use std::str::FromStr;

use log::{error, info};
use solana_sdk::pubkey::Pubkey;
use crate::error::CliResult;
use crate::client::new_client;
use crate::scan::Scan;

pub(crate) fn get_account(address: String) -> CliResult<()> {
    
    let client = new_client();

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

pub(crate) fn trnsfer() ->CliResult<()> {
    let client = new_client();
    // client.send_transaction(transaction)?;
    Ok(())
}

pub(crate) fn load_db(path: String) ->CliResult<()> {
    Ok(())
}

pub(crate) fn scan() -> CliResult<()> {
    let s = Scan::new();
    s.start();
    s.join();
    Ok(())
}