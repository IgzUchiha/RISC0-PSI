use risc0_steel::SolCommitment;

sol! {
    struct Journal {
        SolCommitment commitment;
        address tokenAddress;
    }
}



let journal = Journal {
    commitment: view_call_env.block_commitment(),
    tokenAddress,
};
env::commit_slice(&journal.abi_encode());