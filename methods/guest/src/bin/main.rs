use risc0_zkvm::guest::env;
use risc0_zkvm::serde::{Deserialize, Serialize};

// Define the input struct with serialization/deserialization capabilities.
#[derive(Serialize, Deserialize, Debug)]
struct TransferInput {
    from: [u8; 20],        // Ethereum address of the sender
    to: [u8; 20],          // Ethereum address of the receiver
    token_transfer_value: u32, // Amount to transfer
    balance_from: u32,     // Sender's initial balance
    balance_to: u32,       // Receiver's initial balance
}

impl TransferInput {
    // Logic to verify a single token transfer.
    fn verify_transfer(&self) -> Result<(u32, u32), String> {
        // Ensure the sender has enough balance.
        if self.balance_from < self.token_transfer_value {
            return Err("Insufficient balance".to_string());
        }

        // Perform the token transfer.
        let new_balance_from = self.balance_from - self.token_transfer_value;
        let new_balance_to = self.balance_to + self.token_transfer_value;

        Ok((new_balance_from, new_balance_to))
    }
}

fn main() {
    // Read the input data from the host environment.
    let input: TransferInput = env::read();

    // Perform and verify the transfer logic.
    match input.verify_transfer() {
        Ok((new_balance_from, new_balance_to)) => {
            // Output the new balances as public inputs.
            env::write(&new_balance_from);
            env::write(&new_balance_to);
        }
        Err(err) => {
            // Log the error for debugging.
            env::log(&format!("Verification failed: {}", err));
        }
    }
}
