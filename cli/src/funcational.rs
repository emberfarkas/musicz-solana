use std::ops::Add;
use std::str::FromStr;

use crate::apt::scan_aptos;
use crate::error::CliResult;
// use crate::script_fun_demo::demo_p2p_entry_function;
use log::info;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use web3::ethabi::Address;
use web3::types::{TransactionParameters, TransactionRequest, U256};
use web3::{
    contract::{Contract, Options},
    futures::StreamExt,
    types::FilterBuilder,
};

pub(crate) async fn load_db(path: &String) -> CliResult<()> {
    Ok(())
}

pub(crate) async fn scan() -> CliResult<()> {
    return scan_aptos().await;
}

pub(crate) async fn eth_tx() -> CliResult<()> {
    // htm
    // 0x0Db900d3AECFBB9171bcAd71193DF980F45584ca
    // let transport = web3::transports::Http::new("https://data-seed-prebsc-2-s2.binance.org:8545")?;
    let transport = web3::transports::Http::new("https://http-testnet.hecochain.com")?;
    let web3 = web3::Web3::new(transport);

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x62AB07f83cc62f4bf940D0330f6019588Deed13e")?;

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk =
        SecretKey::from_str("95884f665f4cf15b77a75017b64f9ff7df93b565fb9f3415b4cb352bd627141e")?;

    let secp = Secp256k1::new();
    let pubk = PublicKey::from_secret_key(&secp, &prvk);

    info!("address: {}", pubk.to_string());

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

    info!("Tx succeeded with hash: {}", result);

    Ok(())
}
