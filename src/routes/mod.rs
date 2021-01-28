use std::convert::Infallible;
use std::sync::Arc;

use tokio_postgres::Client;
use uuid::Uuid;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue},
    Filter, Rejection,
};

use crate::auth::Role;

mod attendance;
mod auth;
mod employee;

pub use attendance::*;
pub use auth::*;
pub use employee::*;

fn with_db(db: Arc<Client>) -> impl Filter<Extract = (Arc<Client>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&db))
}

fn with_auth(role: Role) -> impl Filter<Extract = (Uuid,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role, headers))
        .and_then(auth::authorize)
}
