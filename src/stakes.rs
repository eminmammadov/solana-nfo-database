use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::stake;

pub fn get_staker_info(client: &RpcClient) -> Result<(usize, f64), Box<dyn std::error::Error>> {
    let stake_accounts = client.get_program_accounts(&stake::program::id())?;
    let total_stakes = stake_accounts.len();
    let total_stake: u64 = stake_accounts
        .iter()
        .map(|(_, account)| account.lamports)
        .sum();

    Ok((total_stakes, total_stake as f64 / LAMPORTS_PER_SOL as f64))
}