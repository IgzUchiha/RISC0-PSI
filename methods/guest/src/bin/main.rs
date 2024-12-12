// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_doc_comments)]
#![no_main]

use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolValue};
use risc0_steel::{
    ethereum::{EthEvmInput, ETH_SEPOLIA_CHAIN_SPEC},
    Commitment, Contract,
};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

/// Specify the function to call using the [`sol!`] macro.
/// This parses the Solidity syntax to generate a struct that implements the `SolCall` trait.
sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
}

/// ABI encodable journal data.
sol! {
    struct Journal {
        Commitment commitment;
        address tokenAddress;
    }
}
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load the `.env` file into the environment
    dotenv().ok();

    // Retrieve the Alchemy API key from the environment variables
    let alchemy_api_key = env::var("ALCHEMY_API_KEY")
        .expect("ALCHEMY_API_KEY must be set in the environment");

    // Construct the Sepolia endpoint
    let sepolia_url = format!("https://eth-sepolia.g.alchemy.com/v2/{}", alchemy_api_key);

    // Create a provider
    let provider = Provider::<Http>::try_from(sepolia_url)?;

    // Example usage: Fetch the current block number
    let block_number = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);

    Ok(())
}

