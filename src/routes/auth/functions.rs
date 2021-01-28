use std::sync::Arc;

use tokio_postgres::types::Type;
use tokio_postgres::Client;
use warp::{
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, reply, Rejection, Reply,
};

use super::LoginRequest;
use crate::auth::{create_jwt, generate_password, BEARER};
use crate::error::Error;

pub(super) async fn login_handler(
    lr: LoginRequest,
    db: Arc<Client>,
) -> Result<impl Reply, Rejection> {
    let LoginRequest {
        email,
        password,
        permanent,
    } = lr;

    let statement = db
        .prepare_typed(
            "SELECT
            ID, PASSWORD, ROLE
            FROM EMPLOYEE
            WHERE EMAIL = $1",
            &[Type::TEXT],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let row = db
        .query_one(&statement, &[&email])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    let id = row.get(0);
    let password = generate_password(password, &id);
    let stored_password: String = row.get(1);

    if password == stored_password {
        let role: String = row.get(2);
        let role = role
            .parse()
            .map_err(|_| reject::custom(Error::ParseError))?;
        let token = create_jwt(id, role, permanent)
            .map_err(|e| reject::custom(Error::JWTTokenCreationError(e)))?;
        Ok(reply::with_header(
            reply(),
            "set-cookie",
            format!("access_token={}", token),
        ))
    } else {
        Err(reject::custom(Error::WrongCredentialsError))
    }
}

pub(super) fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    let header = headers.get(AUTHORIZATION).ok_or(Error::NoAuthHeaderError)?;
    let auth_header =
        std::str::from_utf8(header.as_bytes()).map_err(|_| Error::NoAuthHeaderError)?;
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
