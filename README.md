# PBA Singapore - Group 14 - Group activity: VRF Card Game Simulation

This Rust project simulates a card game where two players draw each a card. The game ensures fairness by using cryptographic commitments and signatures.

## Overview

The game involves the following steps:

1. **Initialization**: Two players are created with their own seeds for generating key pairs and random values.
2. **Public Key Exchange**: Players disclose their public keys.
3. **Commitment Generation**: Players generate random values and commitments without revealing the values.
4. **Commitment Verification**: Players reveal their random values, and these values are verified to match their commitments.
5. **Common Random Value**: A common random value is generated from the initial random values from both players.
6. **Card Drawing**: Players draw cards using the common random value and sign their drawn cards.
7. **Verification**: The validity of the drawn cards is verified using the public keys and signatures.
8. **Winner Determination**: The winner is determined based on the drawn card values.


# Explanation
This project simulates a poker game where two players draw cards. The game ensures fairness by using cryptographic 
commitments and signatures. Players first reveal their public keys, then generate random values and return commitments
without disclosing their random values. After disclosing their commitments, they reveal their random values.
The commitments are verified to match the random values. A common random value is generated combining both random values,
which serves as input for further functions. Players draw cards by providing a number and a signature of the card value.
Each draw is verified using the previously exchanged public keys and signatures of drawn cards. Finally, the winner is determined
based on the drawn card values.
