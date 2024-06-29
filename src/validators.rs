use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

pub fn get_validator_count(client: &RpcClient) -> Result<usize, Box<dyn std::error::Error>> {
    let validators = client.get_vote_accounts()?;
    Ok(validators.current.len() + validators.delinquent.len())
}

pub fn show_top_validators(client: &RpcClient) -> Result<(), Box<dyn std::error::Error>> {
    let vote_accounts = client.get_vote_accounts()?;
    
    let mut validators: Vec<_> = vote_accounts.current.into_iter()
        .chain(vote_accounts.delinquent.into_iter())
        .collect();

    validators.sort_by(|a, b| b.activated_stake.cmp(&a.activated_stake));

    println!("Top 10 validators with highest stakes:");
    for (i, validator) in validators.iter().take(10).enumerate() {
        println!("{}. Validator {} - Stake: {} SOL",
                 i + 1,
                 validator.vote_pubkey,
                 validator.activated_stake as f64 / LAMPORTS_PER_SOL as f64);
    }

    Ok(())
}