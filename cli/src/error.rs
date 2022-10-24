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
    ParsePubkeyError(solana_sdk::pubkey::ParsePubkeyError, String),
    #[error("sol client error {kind:?}, msg: {msg:?}")]
    SolClientError{
       kind: solana_client::client_error::ClientErrorKind,
       msg: String
    },
}

pub type CliResult<I> = Result<I, CliError>;

impl From<ParsePubkeyError> for CliError {
    fn from(e: ParsePubkeyError) -> Self {
        CliError::ParsePubkeyError(e, "".to_owned())
    }
}

impl From<ClientError> for CliError {
    fn from(e: ClientError) -> Self {
        CliError::SolClientError{kind: e.kind, msg: "kk".to_string()}
    }
}

// impl<T> DecodeError<T> for NameServiceError {
//     fn type_of() -> &'static str {
//         "NameServiceError"
//     }
// }
