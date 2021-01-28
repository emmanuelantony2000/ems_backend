use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid: {0}")]
    JWTTokenError(jsonwebtoken::errors::Error),
    #[error("jwt token creation error: {0}")]
    JWTTokenCreationError(jsonwebtoken::errors::Error),
    #[error("jwt expired error")]
    JWTExpiredError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("parse error")]
    ParseError,
    #[error("database connect error: {0}")]
    DBConnectError(tokio_postgres::error::Error),
    #[error("db statement prepare error: {0}")]
    StatementPrepareError(tokio_postgres::error::Error),
    #[error("db statement query error: {0}")]
    QueryError(tokio_postgres::error::Error),
    #[error("db statement execute error: {0}")]
    ExecuteError(tokio_postgres::error::Error),
    #[error("db insert unsuccessful, succeeded: {0} failed: {1}")]
    InsertUnsuccessfulError(u64, u64),
    #[error("attendance already exists")]
    DuplicateAttendanceError,
}

impl warp::reject::Reject for Error {}
