use solana_client::rpc_client::RpcClient;

// const DEV_NET : &str = "https://rpc.ankr.com/solana_devnet";
const DEV_NET : &str = "https://api.devnet.solana.com";

pub(crate) fn new_client() -> RpcClient {
    let url = DEV_NET.to_string();
    RpcClient::new(url)
}