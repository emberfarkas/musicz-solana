use solana_client::client_error::ClientError;

use {
    solana_sdk::pubkey::ParsePubkeyError,
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Out of space")]
    OutOfSpace,
    #[error("parse pubkey error")]
    ParsePubkeyError(solana_sdk::pubkey::ParsePubkeyError),
    #[error("client error")]
    ClientError(solana_client::client_error::ClientErrorKind),
}

pub type CliResult<I> = Result<I, CliError>;

impl From<ParsePubkeyError> for CliError {
    fn from(e: ParsePubkeyError) -> Self {
        CliError::ParsePubkeyError(e)
    }
}

impl From<ClientError> for CliError {
    fn from(e: ClientError) -> Self {
        CliError::ClientError(e.kind)
    }
}

// impl<T> DecodeError<T> for NameServiceError {
//     fn type_of() -> &'static str {
//         "NameServiceError"
//     }
// }
