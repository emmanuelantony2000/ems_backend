use std::sync::Arc;

use tokio_postgres::types::Type;
use tokio_postgres::Client;
use uuid::Uuid;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, reply, Filter, Rejection, Reply,
};

use super::with_db;
use crate::auth::{create_jwt, decode, generate_password, Role, JWT_SECRET};
use crate::error::Error;

mod data_structures;
mod functions;

use data_structures::LoginRequest;

pub fn login(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(login_handler)
}

pub(super) async fn authorize(
    (role, headers): (Role, HeaderMap<HeaderValue>),
) -> Result<Uuid, Rejection> {
    let jwt = functions::jwt_from_header(&headers)?;
    let claims = decode(jwt, JWT_SECRET).map_err(|e| reject::custom(Error::JWTTokenError(e)))?;

    let c_role: Role = claims
        .role
        .parse()
        .map_err(|_| reject::custom(Error::ParseError))?;

    if role != c_role {
        return Err(reject::custom(Error::NoPermissionError));
    }

    Ok(Uuid::parse_str(&claims.sub).map_err(|_| reject::custom(Error::ParseError))?)
}

pub async fn login_handler(lr: LoginRequest, db: Arc<Client>) -> Result<impl Reply, Rejection> {
    let LoginRequest { email, password } = lr;

    let statement = db
        .prepare_typed(
            "SELECT
            ID, PASSWORD, ROLE
            FROM EMPLOYEE
            WHERE EMAIL = $1",
            &[Type::TEXT],
        )
        .await
        .map_err(|_| reject::custom(Error::StatementPrepareError))?;

    let row = db
        .query_one(&statement, &[&email])
        .await
        .map_err(|_| reject::custom(Error::QueryError))?;

    let id = row.get(0);
    let password = generate_password(password, &id);
    let stored_password: String = row.get(1);

    if password == stored_password {
        let role: String = row.get(2);
        let role = role
            .parse()
            .map_err(|_| reject::custom(Error::ParseError))?;
        let token =
            create_jwt(id, role).map_err(|e| reject::custom(Error::JWTTokenCreationError(e)))?;
        Ok(reply::with_header(
            reply(),
            "set-cookie",
            format!("access_token={}", token),
        ))
    } else {
        Err(reject::custom(Error::WrongCredentialsError))
    }
}
