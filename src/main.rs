use sp_core::ed25519::{Pair as Ed25519Pair, Signature as Ed25519Signature, Public as Ed25519Public};
use sp_core::{blake2_128, Pair};
use rand::{Rng, rngs::SmallRng, SeedableRng};
use sp_core::hexdisplay::AsBytesRef;
use sp_runtime::traits::{Verify};

const HASH_SIZE: usize = 16;
/// Use the blake2 hashing algorithm to calculate the 128-bit hash of some input data
pub fn hash_with_blake(data: &[u8]) -> [u8; HASH_SIZE] {
    blake2_128(data)
}

fn main() {
    const SEED1: &str = "source upgrade van toy cross smooth write erupt uncover today injury say wealth silk thought slide shadow comfort hazard planet wisdom problem review pudding";
    const SEED2: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

    let mut player1 = Player::new(SEED1, 78u64);
    let mut player2 = Player::new(SEED2, 5u64);

    // players disclose their public keys
    let pk1 = player1.pk();
    let pk2 = player2.pk();

    // players generate random values (at the moment kept to themselves)
    // and return their commitments
    let commitment1 = player1.generate_commitment_initial_value();
    let commitment2 = player2.generate_commitment_initial_value();

    // players return their values
    let random_init_1 = player1.get_generated_random_initial_value().expect("Random initial value (and thus commitment) has not been generated");
    let random_init_2 = player2.get_generated_random_initial_value().expect("Random initial value (and thus commitment) has not been generated");

    // check values match commitments
    if hash_with_blake(random_init_1.as_bytes_ref()) != commitment1 {
        println!("player 1 cheated");
        return;
    }
    if hash_with_blake(random_init_2.as_bytes_ref()) != commitment2 {
        println!("player 2 cheated");
        return;
    }

    // make common consensus random input
    let random_common = Player::combine_random_inits(random_init_1, random_init_2);

    // players draw cards along with signatures
    let (signature1, card1) = player1.draw_card(random_common);
    let (signature2, card2) = player2.draw_card(random_common);

    // verify VRF of the cards
    if Player::verify_draw(random_common, signature1, pk1, card1).is_err() {
        println!("player 1 cheated");
    }
    if Player::verify_draw(random_common, signature2, pk2, card2).is_err() {
        println!("player 2 cheated");
    }

    // check final result
    if card1 > card2 {
        println!("Player 1 wins");
    } else if card1 < card2 {
        println!("Player 2 wins");
    } else {
        println!("Draw");
    }
}

struct Player {
    pair: Ed25519Pair,
    commital: Option<[u8; 4]>,
    rng: SmallRng,
}




impl Player {
    fn new(seed: &str, rng_seed: u64) -> Player {
        let pair = Ed25519Pair::from_phrase(seed, None).unwrap().0;
        Self {
            pair,
            commital: None,
            rng: SmallRng::seed_from_u64(rng_seed),
        }
    }

    pub fn pk(&self) -> Ed25519Public {
        self.pair.public()
    }

    pub fn generate_commitment_initial_value(&mut self) -> [u8; HASH_SIZE] {
        let randomness: [u8; 4] = self.rng.gen();
        let hash = hash_with_blake(randomness.as_bytes_ref());
        self.commital = Some(randomness);
        hash
    }

    pub fn get_generated_random_initial_value(&mut self) -> Option<[u8; 4]> {
        self.commital
    }

    pub fn combine_random_inits(random_init_1: [u8; 4], random_init_2: [u8; 4]) -> [u8; HASH_SIZE] {
        let mut combined = [0u8; 8]; // Create a new array with 8 elements
        combined[..4].copy_from_slice(&random_init_1); // Copy the elements of arr1 into the first half
        combined[4..].copy_from_slice(&random_init_2); // Copy the elements of arr2 into the second half
        hash_with_blake(combined.as_bytes_ref())
    }

    pub fn compute_card(signature: &Ed25519Signature) -> u128 {
        let hash_bytes = hash_with_blake(signature.as_ref());
        u128::from_be_bytes(hash_bytes) % 52
    }

    pub fn draw_card(&self, input: [u8; HASH_SIZE]) -> (Ed25519Signature, u128) {
        let signature = self.pair.sign(input.as_bytes_ref());
        let card = Self::compute_card(&signature);
        (signature, card)
    }

    pub fn verify_draw(input: [u8; HASH_SIZE], signature: Ed25519Signature, pk: Ed25519Public, card: u128) -> Result<(), ()> {
        let input_ref = input.as_ref();
        if !signature.verify(input_ref, &pk) { return Err(()); }
        if Self::compute_card(&signature) != card { return Err(()); }
        Ok(())
    }
}