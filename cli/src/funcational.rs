use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use crate::error::CliResult;

pub fn get_account() -> CliResult<()> {
    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new(url);

    // 成功返回alice_pubkey, err返回err
    let alice_pubkey = Pubkey::from_str("BgvYtJEfmZYdVKiptmMjxGzv8iQoo4MWjsP3QsTkhhxa")?;

    let account = client.get_account(&alice_pubkey)?;
    println!("{}", account.owner.to_string());
    Ok(())
}