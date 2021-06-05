use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentials,

    #[error("jwt token creation error")]
    JWTTokenCreation,

    #[error("no auth header")]
    InvalidAuthHeader,

    #[error("no permission")]
    NoPermission,

    #[error("internal server error")]
    ServerError,

    #[error("user not found")]
    UserNotFound,
}

impl ErrorExtensions for Error {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            Error::WrongCredentials  => e.set("code", "UNAUTHORIZED"),
            Error::JWTTokenCreation  => e.set("code", "INTERNAL_SERVER_ERROR"),
            Error::InvalidAuthHeader => e.set("code", "UNAUTHORIZED"),
            Error::NoPermission      => e.set("code", "UNAUTHORIZED"),
            Error::ServerError       => e.set("code", "INTERNAL_SERVER_ERROR"),
            Error::UserNotFound      => e.set("code", "NOT_FOUND"),
        })
    }
}