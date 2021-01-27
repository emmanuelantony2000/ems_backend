use std::convert::Infallible;
use std::sync::Arc;

use tokio_postgres::Client;
use uuid::Uuid;
use warp::Filter;

use super::{with_auth, with_db};
use crate::auth::Role;

mod data_structures;
mod functions;

use data_structures::{Employee, EmployeeId};

pub fn get_employee(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee" / Uuid)
        .and(warp::get())
        .and(with_db(db))
        .and(with_auth(Role::Admin))
        .map(|id, db, _| (id, db))
        .and_then(functions::ge)
}

pub fn get_self(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee" / "self")
        .and(warp::get())
        .and(with_db(db))
        .and(with_auth(Role::User))
        .map(|db, id| (id, db))
        .and_then(functions::ge)
}

pub fn get_employees(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee")
        .and(warp::get())
        .and(with_db(db))
        .and(with_auth(Role::Admin))
        .map(|db, _| db)
        .and_then(functions::ges)
}

pub fn post_employees(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and(with_auth(Role::Admin))
        .map(|employees, db, _| (employees, db))
        .and_then(functions::pe)
}
