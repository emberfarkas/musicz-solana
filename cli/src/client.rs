
use once_cell::sync::Lazy;
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use aptos_sdk::coin_client::CoinClient;
use aptos_sdk::rest_client::{Client, FaucetClient};
use aptos_sdk::types::LocalAccount;
use url::Url;

// const DEV_NET : &str = "https://rpc.ankr.com/solana_devnet";
const DEV_NET: &str = "https://api.devnet.solana.com";

pub(crate) fn new_solana_client() -> RpcClient {
    let url = DEV_NET.to_string();
    RpcClient::new(url)
}

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

pub(crate) fn new_aptos_rest_client() ->Client {
    Client::new(NODE_URL.clone())
}