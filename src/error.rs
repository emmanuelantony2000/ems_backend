use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError(jsonwebtoken::errors::Error),
    #[error("jwt token creation error")]
    JWTTokenCreationError(jsonwebtoken::errors::Error),
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("parse error")]
    ParseError,
    #[error("db statement prepare error")]
    StatementPrepareError,
    #[error("db statement query error")]
    QueryError,
    #[error("db statement execute error")]
    ExecuteError,
    #[error("response creation error")]
    ResponseCreationError,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}
