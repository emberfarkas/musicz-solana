use std::str::FromStr;

use log::info;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::error::CliResult;

// const DEV_NET : &str = "https://rpc.ankr.com/solana_devnet";
const DEV_NET: &str = "https://api.devnet.solana.com";

fn new_solana_client() -> RpcClient {
    let url = DEV_NET.to_string();
    RpcClient::new(url)
}

pub(crate) async fn get_account(address: &String) -> CliResult<()> {
    info!("get account for address");

    let client = new_solana_client();

    let block_height = client.get_block_height()?;
    info!("block height {}", block_height);

    // 成功返回alice_pubkey, err返回err
    // let alice_pubkey = Pubkey::from_str("32sicBwphxYCwXBYKQecm7HKdBtytGn9RqnHJcezN7b7")?;
    let alice_pubkey = Pubkey::from_str(address.as_str())?;

    let balance = client.get_balance(&alice_pubkey)?;
    info!("accout[{}] {}", alice_pubkey.to_string(), balance);

    let slot = client.get_slot()?;
    info!("slot {}", slot);

    // let token_balance = client.get_token_account_balance(&alice_pubkey)?;
    // println!("token accout[{}] {}", alice_pubkey.to_string(), token_balance.amount);

    Ok(())
}

pub(crate) async fn trnsfer() -> CliResult<()> {
    let client = new_solana_client();
    // client.send_transaction(transaction)?;

    // demo_p2p_entry_function();
    Ok(())
}

pub(crate) async fn scan_solana() -> CliResult<()> {
    let client = new_solana_client();
    let mut block_height = 1;
    tokio::spawn(async move {
        loop {
            match client.get_block(block_height) {
                Err(e) => info!("{}", e),
                Ok(block) => info!("{}", block.blockhash),
            }
            block_height = block_height + 1;
        }
    });
    Ok(())
}
