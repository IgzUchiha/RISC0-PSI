use risc0_zkvm::guest::env;
use risc0_zkvm::serde::{Deserializer, Serializer};
// use risc0_zkvm::serde::{from_slice, to_vec};

#[derive(Serializer, Deserializer, Debug)]
struct TransferInput {
    from: [u8; 20],        // Ethereum address of the sender
    to: [u8; 20],          // Ethereum address of the receiver
    token_transfer_value: u32, // Amount to transfer
    balance_from: u32,     // Sender's initial balance
    balance_to: u32,       // Receiver's initial balance
}

impl TransferInput {
    fn verify_transfer(&self) -> Result<(u32, u32), String> {
        if self.balance_from < self.token_transfer_value {
            return Err("Insufficient balance".to_string());
        }

        let new_balance_from = self.balance_from - self.token_transfer_value;
        let new_balance_to = self.balance_to + self.token_transfer_value;

        Ok((new_balance_from, new_balance_to))
    }
}

fn main() {
    let input: TransferInput = env::read();

    match input.verify_transfer() {
        Ok((new_balance_from, new_balance_to)) => {
            env::write(&new_balance_from);
            env::write(&new_balance_to);
        }
        Err(err) => {
            env::log(&format!("Verification failed: {}", err));
        }
    }
}
