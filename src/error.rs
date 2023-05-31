use secp256k1::Error as Secp256k1Error;
use solana_client::client_error::ClientError;
use thiserror::Error;
use tokio::task::JoinError;
use web3::error::Error as Web3Error;
use {fixed_hash::rustc_hex::FromHexError, solana_sdk::pubkey::ParsePubkeyError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Out of space")]
    OutOfSpace,
    #[error("String is the wrong size")]
    ParsePubkeyWrongSize,
    #[error("Invalid Base58 string")]
    ParsePubkeyInvalid,
    #[error("sol client error {kind:?}, msg: {msg:?}")]
    SolClientError {
        kind: solana_client::client_error::ClientErrorKind,
        msg: String,
    },
    #[error("web3 client error msg: {msg:?}")]
    Web3Error { msg: String },
    #[error("join error msg: {msg:?}")]
    JoinError { msg: String },
}

pub type CliResult<I> = Result<I, CliError>;

impl From<ParsePubkeyError> for CliError {
    fn from(e: ParsePubkeyError) -> Self {
        match e {
            ParsePubkeyError::WrongSize => CliError::ParsePubkeyWrongSize,
            ParsePubkeyError::Invalid => CliError::ParsePubkeyInvalid,
        }
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
        CliError::Web3Error { msg: e.to_string() }
    }
}

impl From<Secp256k1Error> for CliError {
    fn from(e: Secp256k1Error) -> Self {
        CliError::Web3Error { msg: e.to_string() }
    }
}

impl From<FromHexError> for CliError {
    fn from(e: FromHexError) -> Self {
        CliError::Web3Error { msg: e.to_string() }
    }
}

impl From<JoinError> for CliError {
    fn from(e: JoinError) -> Self {
        CliError::JoinError { msg: e.to_string() }
    }
}
