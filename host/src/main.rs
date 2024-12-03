use risc0_zkvm::prover::Prover;
use risc0_zkvm::serde::{to_vec}; // For serialization
use methods::{TRANSFER_METHOD_ID, TransferInput};

fn main() {
    // Define the input to the guest
    let input = TransferInput {
        from: [0; 20],               // Sender address
        to: [1; 20],                 // Receiver address
        token_transfer_value: 100,   // Transfer amount
        balance_from: 1000,          // Sender's initial balance
        balance_to: 500,             // Receiver's initial balance
    };

    // Serialize the input using risc0_zkvm's serde
    let input_data = to_vec(&input).expect("Serialization failed");

    // Create a prover and add serialized input
    let mut prover = Prover::new(TRANSFER_METHOD_ID).expect("Prover creation failed");
    prover.add_input_u8_slice(&input_data);

    // Generate the proof
    let proof = prover.run().expect("Proof generation failed");

    // Use `proof` and `input_data` for on-chain verification
    println!("Proof generated successfully.");
}

