use std::str::FromStr;

use log::{error, info};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use crate::error::CliResult;

// const DEV_NET : &str = "https://rpc.ankr.com/solana_devnet";
const DEV_NET : &str = "https://api.devnet.solana.com";

fn new_client() -> RpcClient {
    let url = DEV_NET.to_string();
    RpcClient::new(url)
}

pub fn get_account() -> CliResult<()> {
    
    let client = new_client();

    let block_height = client.get_block_height()?;
    info!("block height {}", block_height);

    // 成功返回alice_pubkey, err返回err
    let alice_pubkey = Pubkey::from_str("32sicBwphxYCwXBYKQecm7HKdBtytGn9RqnHJcezN7b7")?;

    let balance = client.get_balance(&alice_pubkey)?;
    println!("accout[{}] {}", alice_pubkey.to_string(), balance);

    let slot = client.get_slot()?;
    println!("slot {}", slot);

    // let token_balance = client.get_token_account_balance(&alice_pubkey)?;
    // println!("token accout[{}] {}", alice_pubkey.to_string(), token_balance.amount);

    Ok(())
}

pub fn trnsfer() ->CliResult<()> {
    let client = new_client();
    // client.send_transaction(transaction)?;
    Ok(())
}