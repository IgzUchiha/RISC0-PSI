use alloy_sol_types::{sol, SolValue};
use risc0_zkvm::guest::env;
use alloy_primitives::Address;

// sol! {
//     struct Journal {
//         commitment: BlockCommitment, // Ensure BlockCommitment is SolType-compatible
//         tokenAddress: Address,
//     }
// }
pub struct Journal {
    pub bytes: Vec<u8>,
}

fn main() {
    let token_address = Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap();
    let journal = Journal {
        commitment: BlockCommitment::default(),
        tokenAddress: token_address,
    };

    // Encode and commit the journal
    env::commit_slice(&journal.abi_encode());
}
