use aptos_sdk::rest_client::{Client, FaucetClient};
use log::info;
use once_cell::sync::Lazy;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use url::Url;

use crate::error::{CliError, CliResult};

// // :!:>section_1c
static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
    )
    .unwrap()
});

static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_FAUCET_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://faucet.devnet.aptoslabs.com"),
    )
    .unwrap()
});
// <:!:section_1c

pub(crate) fn new_aptos_rest_client() -> Client {
    Client::new(NODE_URL.clone())
}

pub(crate) async fn scan_aptos() -> CliResult<()> {
    let client = new_aptos_rest_client();
    let mut block_height = 1;
    let ret = tokio::spawn(async move {
        loop {
            match client.get_block_by_height(block_height, true).await {
                Err(e) => info!("{}", e),
                Ok(res) => {
                    block_height = block_height + 1;
                    info!("chainID: {}", res.state().chain_id);
                }
            }
        }
    })
    .await;
    match ret {
        Err(e) => Err(e.into()),
        Ok(_) => Ok(()),
    }
}

pub(crate) async fn transfer_aptos(address: String) -> CliResult<()> {
    let client = new_aptos_rest_client();

    // 成功返回alice_pubkey, err返回err
    let alice_pubkey = Pubkey::from_str("32sicBwphxYCwXBYKQecm7HKdBtytGn9RqnHJcezN7b7")?;
    let alice_pubkey = Pubkey::from_str(address.as_str())?;

    // let balance = client.get_balance(&alice_pubkey)?;
    // info!("accout[{}] {}", alice_pubkey.to_string(), balance);

    // let slot = client.get_slot()?;
    // info!("slot {}", slot);

    // let token_balance = client.get_token_account_balance(&alice_pubkey)?;
    // println!("token accout[{}] {}", alice_pubkey.to_string(), token_balance.amount);

    Ok(())
}
