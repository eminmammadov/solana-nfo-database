mod validators;
mod stakes;
mod cli;

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_cli_matches();

    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let timeout = Duration::from_secs(60);
    let client = RpcClient::new_with_timeout_and_commitment(rpc_url, timeout, CommitmentConfig::confirmed());

    if matches.is_present("validators") {
        let validator_count = validators::get_validator_count(&client)?;
        println!("Total number of validators: {}", validator_count);
    }

    if matches.is_present("stakes") {
        let (total_stakes, total_stake) = stakes::get_staker_info(&client)?;
        println!("Total number of stakes: {}", total_stakes);
        println!("Total staked amount: {} SOL", total_stake);
    }

    if matches.is_present("top-validators") {
        validators::show_top_validators(&client)?;
    }

    Ok(())
}