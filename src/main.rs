use schnorrkel::{
    context::SigningContext,
    signing_context,
    vrf::{VRFInOut, VRFProof, VRFProofBatchable},
    Keypair, Signature,
};

// Card draw function (replace with your game logic to determine input)
fn get_card_draw_input(player_id: u8, game_state: u64) -> String {
    // format!("{}{}", player_id, game_state)
    return "10".to_string();
}

// Simulate drawing cards (replace with actual card selection logic)
fn draw_card(
    player_secret: &Keypair,
    ctx: SigningContext,
    input: &str,
) -> (VRFInOut, VRFProof, VRFProofBatchable) {
    player_secret.vrf_sign(ctx.bytes(input.as_bytes()))
}

fn main() {
    let mut csprng = rand_core::OsRng;

    let player1_secret = Keypair::generate_with(&mut csprng);
    let player2_secret = Keypair::generate_with(&mut csprng);

    let ctx: schnorrkel::context::SigningContext = signing_context(b"good");

    // Players draw cards
    let player1_input = get_card_draw_input(1, 10);
    let (io1, proof1, proof1batchable) = draw_card(&player1_secret, ctx.clone(), &player1_input);
    let out1 = &io1.to_preout();

    let player2_input = get_card_draw_input(2, 10);
    let (io2, proof2, proof2batchable) = draw_card(&player2_secret, ctx.clone(), &player2_input);
    let out2 = &io2.to_preout();

    // Reveal card (when needed)
    let player1_card = player1_secret
        .public
        .vrf_verify(ctx.bytes(b"10"), &out1, &proof1);

    let player2_card = player2_secret
        .public
        .vrf_verify(ctx.bytes(b"10"), &out2, &proof2);

    print!("player1_card: {:?}", player1_card.unwrap().0);
    print!("player2_card: {:?}", player2_card.unwrap().0);
    // Anyone can verify the card using player1's public key (obtained from player1_secret) and Schnorrkel's verification function
    // ... (verification logic not shown here) ...
}
