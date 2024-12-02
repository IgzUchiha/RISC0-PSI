use risc0_zkvm::prover::prove;
use risc0_zkvm::types::u256;
use crate::guest::TransferInput;

fn main() {
    let input = TransferInput {
        from: [0; 20],
        to: [1; 20],
        value: 100,
        balance_from: 1000,
        balance_to: 500,
    };

    // Generate proof
    let proof = prove(input).expect("Proof generation failed");
    
    // At this point, you can send `proof` to your on-chain verifier
}
