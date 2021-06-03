use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,

    #[error("jwt token creation error")]
    JWTTokenCreationError,

    #[error("no auth header")]
    InvalidAuthHeaderError,

    #[error("no permission")]
    NoPermissionError,
}