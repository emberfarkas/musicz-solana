use std::str::FromStr;

use crate::client::{new_solana_client, new_aptos_rest_client};
use crate::error::CliResult;
use crate::scan::Scan;
use log::{error, info};
use solana_sdk::pubkey::Pubkey;
use crate::script_fun_demo::demo_p2p_entry_function;

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

    demo_p2p_entry_function();
    Ok(())
}

pub(crate) async fn load_db(path: String) -> CliResult<()> {
    Ok(())
}

pub(crate) async fn scan_solana() -> CliResult<()> {
    let client = new_solana_client();
    let mut block_height = 1;
    tokio::spawn( async move {
        loop {
            match client.get_block(block_height) {
                Err(e) => info!("{}", e),
                Ok(block) => info!("{}", block.blockhash),
            }
            block_height =block_height+1;
        }    
    });
    Ok(())
}

pub(crate) async fn scan_aptos() -> CliResult<()> {
    let client = new_aptos_rest_client();
    let mut block_height = 1;
    tokio::spawn( async move {
        loop {
            match client.get_block_by_height(block_height, true).await {
                Err(e) => info!("{}", e),
                Ok(res) => {
                    info!("{}", res.state().chain_id);
                },
            }
            block_height =block_height+1;
        }    
    }).await;
    Ok(())
}


pub(crate) async fn scan() -> CliResult<()> {
    return  scan_aptos().await;
}

pub(crate) async fn get_aptos(address: String) -> CliResult<()> {
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