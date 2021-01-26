use std::convert::Infallible;

use anyhow::bail;
use warp::http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use warp::Reply;

use crate::auth::{decode, Role, BEARER, JWT_SECRET};

macro_rules! bail {
    ($res:ident) => {
        match $res {
            Ok(x) => x,
            Err(e) => {
                return Ok(reply::with_status(
                    format!("{:?}", e),
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response());
            }
        };
    };
}

async fn authorize(
    (role, headers): (Role, HeaderMap<HeaderValue>),
) -> Result<impl Reply, Infalliable> {
    let jwt = jwt_from_header(&headers);
    let jwt = bail!(jwt);

    let claims = decode(jwt, JWT_SECRET);
    let claims = bail!(claims);
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> anyhow::Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => bail!("No auth header"),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => bail!("No auth header"),
    };
    if !auth_header.starts_with(BEARER) {
        return bail!("Invalid auth header");
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
