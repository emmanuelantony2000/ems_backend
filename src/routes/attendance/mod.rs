use std::sync::Arc;

use tokio_postgres::Client;
use warp::Filter;

use super::{with_auth, with_db};
use crate::auth::Role;

mod data_structures;
mod functions;

use data_structures::Attendance;

pub fn get_attendance(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("attendance")
        .and(warp::get())
        .and(with_db(db))
        .and(with_auth(Role::User))
        .map(|db, id| (id, db))
        .and_then(functions::ga)
}

pub fn post_attendance(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("attendance")
        .and(warp::post())
        .and(with_db(db))
        .and(with_auth(Role::User))
        .map(|db, id| (id, db))
        .and_then(functions::pa)
}
