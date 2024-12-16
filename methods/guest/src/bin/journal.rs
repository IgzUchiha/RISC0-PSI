// use alloy_sol_types::{sol, SolValue};
// use risc0_zkvm::guest::env;
// use alloy_primitives::Address;

// // sol! {
// //     struct Journal {
// //         commitment: BlockCommitment, // Ensure BlockCommitment is SolType-compatible
// //         tokenAddress: Address,
// //     }
// // }
// pub struct Journal {
//     pub bytes: Vec<u8>,
// }

// fn main() {
//     let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();
//     let journal = Journal {
//         commitment: BlockCommitment::default(),
//         tokenAddress: token_address,
//     };

//     // Encode and commit the journal
//     env::commit_slice(&journal.abi_encode());
// }
use alloy_primitives::Address;
use risc0_zkvm::guest::env;
use risc0_zkvm::Journal;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)] // Added Debug
struct Commitment {
    data: String, // Example field
}

fn main() {
    // Create a mock commitment
    let commitment = Commitment {
        data: "Example commitment".to_string(),
    };

    // Serialize the commitment into bytes
    let commitment_bytes = bincode::serialize(&commitment).expect("Serialization failed");

    // Use a token address
    let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678")
        .expect("Invalid address");

    // Combine commitment and address into a single byte vector
    let mut journal_bytes = commitment_bytes.clone();
    journal_bytes.extend(token_address.as_fixed_bytes()); // Correct method

    // Create a Journal
    let journal = Journal::new(journal_bytes);

    // Commit the Journal
    env::commit_slice(journal.as_ref());

    // Decode (optional, for debugging or further processing)
    let decoded_commitment: Commitment = bincode::deserialize(&journal.bytes)
        .expect("Decoding failed");
    println!("Decoded Commitment: {:?}", decoded_commitment);
}
