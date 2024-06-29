use clap::{App, Arg};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Solana Info CLI")
        .version("1.1.1")
        .author("Emin Mammadov")
        .about("Provides information about the Solana network")
        .arg(
            Arg::with_name("validators")
                .long("validators")
                .help("Number of validators"),
        )
        .arg(
            Arg::with_name("stakes")
                .long("stakes")
                .help("Number of stakes and total stake amount"),
        )
        .arg(
            Arg::with_name("top-validators")
                .long("top-validators")
                .help("Top 10 validators with the highest stakes"),
        )
        .get_matches();

    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let timeout = Duration::from_secs(60);
    let client = RpcClient::new_with_timeout_and_commitment(rpc_url, timeout, CommitmentConfig::confirmed());

    if matches.is_present("validators") {
        let validators = client.get_vote_accounts()?;
        println!("Total number of validators
: {}", validators.current.len() + validators.delinquent.len());
    }

    if matches.is_present("stakes") {
        let stake_accounts = client.get_program_accounts(&solana_sdk::stake::program::id())?;
        let total_stakers = stake_accounts.len();
        let total_stake: u64 = stake_accounts
            .iter()
            .map(|(_, account)| account.lamports)
            .sum();

        println!("Total number of stakers: {}", total_stakers);
        println!("Total staked amount: {} SOL", total_stake as f64 / LAMPORTS_PER_SOL as f64);
    }

    if matches.is_present("top-validators") {
        show_top_validators(&client)?;
    }

    Ok(())
}

fn show_top_validators(client: &RpcClient) -> Result<(), Box<dyn std::error::Error>> {
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