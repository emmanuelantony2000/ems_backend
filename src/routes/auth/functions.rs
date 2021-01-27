use std::convert::Infallible;
use std::sync::Arc;

use tokio_postgres::types::Type;
use tokio_postgres::Client;
use uuid::Uuid;
use warp::http::{
    self,
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
};
use warp::{reject, reply, Rejection, Reply};

use super::LoginRequest;
use crate::auth::{create_jwt, decode, generate_password, Role, BEARER, JWT_SECRET};
use crate::error::Error;

pub(super) fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    let header = headers.get(AUTHORIZATION).ok_or(Error::NoAuthHeaderError)?;
    let auth_header =
        std::str::from_utf8(header.as_bytes()).map_err(|_| Error::NoAuthHeaderError)?;
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
