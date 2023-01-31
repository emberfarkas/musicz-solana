use std::str::FromStr;

use crate::client::{new_aptos_rest_client, new_solana_client};
use crate::error::CliResult;
use crate::scan::Scan;
// use crate::script_fun_demo::demo_p2p_entry_function;
use log::{error, info};
use secp256k1::SecretKey;
use solana_sdk::pubkey::Pubkey;
use web3::ethabi::Address;
use web3::types::{TransactionParameters, TransactionRequest, U256};
use web3::{
    contract::{Contract, Options},
    futures::StreamExt,
    types::FilterBuilder,
};

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

pub(crate) async fn load_db(path: &String) -> CliResult<()> {
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

pub(crate) async fn scan_aptos() -> CliResult<()> {
    let client = new_aptos_rest_client();
    let mut block_height = 1;
    tokio::spawn(async move {
        loop {
            match client.get_block_by_height(block_height, true).await {
                Err(e) => info!("{}", e),
                Ok(res) => {
                    info!("{}", res.state().chain_id);
                }
            }
            block_height = block_height + 1;
        }
    })
    .await;
    Ok(())
}

pub(crate) async fn scan() -> CliResult<()> {
    return scan_aptos().await;
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

pub(crate) async fn eth_tx() -> CliResult<()> {
    // htm
    // 0x0Db900d3AECFBB9171bcAd71193DF980F45584ca
    let transport = web3::transports::Http::new("https://data-seed-prebsc-2-s2.binance.org:8545")?;
    let web3 = web3::Web3::new(transport);

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x62AB07f83cc62f4bf940D0330f6019588Deed13e")?;

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk =
        SecretKey::from_str("95884f665f4cf15b77a75017b64f9ff7df93b565fb9f3415b4cb352bd627141e")?;

    // Build the tx object
    let tx_object = TransactionParameters {
        to: Some(to),
        value: U256::exp10(17), //0.1 eth
        ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    // Send the tx to infura
    let result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;

    println!("Tx succeeded with hash: {}", result);

    Ok(())
}
