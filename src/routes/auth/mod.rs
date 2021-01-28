use std::sync::Arc;

use chrono::Utc;
use tokio_postgres::Client;
use uuid::Uuid;
use warp::{
    http::header::{HeaderMap, HeaderValue},
    reject, Filter, Rejection,
};

use super::with_db;
use crate::auth::{decode, Role, JWT_SECRET};
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
        .and_then(functions::login_handler)
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
    let exp = claims.exp;
    let now = Utc::now().timestamp() as usize;

    if exp < now {
        return Err(reject::custom(Error::JWTExpiredError));
    }

    if role != c_role {
        return Err(reject::custom(Error::NoPermissionError));
    }

    Ok(Uuid::parse_str(&claims.sub).map_err(|_| reject::custom(Error::ParseError))?)
}
