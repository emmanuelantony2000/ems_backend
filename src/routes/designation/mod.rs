use std::sync::Arc;

use tokio_postgres::Client;
use warp::Filter;

use crate::auth::Role;
use crate::routes::{with_auth, with_db};

mod data_structures;
mod functions;

use data_structures::Designation;

pub fn get_designation(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("designation")
        .and(warp::get())
        .and(with_db(db))
        .and(with_auth(Role::User))
        .map(|db, id| (id, db))
        .and_then(functions::gd)
}

pub fn post_designation(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("designation")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and(with_auth(Role::Admin))
        .map(|designations, db, _| (designations, db))
        .and_then(functions::pd)
}

pub fn get_designations(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("designation")
        .and(warp::get())
        .and(with_db(db))
        .and(with_auth(Role::Admin))
        .map(|db, _| db)
        .and_then(functions::gda)
}

pub fn post_designations(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("designation")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and(with_auth(Role::Admin))
        .map(|designations, db, _| (designations, db))
        .and_then(functions::pda)
}
