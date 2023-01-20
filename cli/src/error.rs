use solana_client::client_error::ClientError;

use {
    fixed_hash::rustc_hex::FromHexError, secp256k1::Error as Secp256k1Error,
    solana_sdk::pubkey::ParsePubkeyError, thiserror::Error, web3::error::Error as Web3Error,
};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Out of space")]
    OutOfSpace,
    #[error("parse pubkey error")]
    ParsePubkeyError(solana_sdk::pubkey::ParsePubkeyError, String),
    #[error("sol client error {kind:?}, msg: {msg:?}")]
    SolClientError {
        kind: solana_client::client_error::ClientErrorKind,
        msg: String,
    },
    #[error("parse pubkey error")]
    Web3Error,
}

pub type CliResult<I> = Result<I, CliError>;

impl From<ParsePubkeyError> for CliError {
    fn from(e: ParsePubkeyError) -> Self {
        CliError::ParsePubkeyError(e, "".to_owned())
    }
}

impl From<ClientError> for CliError {
    fn from(e: ClientError) -> Self {
        CliError::SolClientError {
            kind: e.kind,
            msg: "kk".to_string(),
        }
    }
}

impl From<Web3Error> for CliError {
    fn from(e: Web3Error) -> Self {
        CliError::Web3Error
    }
}

impl From<Secp256k1Error> for CliError {
    fn from(e: Secp256k1Error) -> Self {
        CliError::Web3Error
    }
}

impl From<FromHexError> for CliError {
    fn from(e: FromHexError) -> Self {
        CliError::Web3Error
    }
}
