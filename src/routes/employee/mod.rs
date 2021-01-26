use std::convert::Infallible;
use std::sync::Arc;

use tokio_postgres::Client;
use uuid::Uuid;
use warp::Filter;

mod data_structures;
mod functions;

use data_structures::{Employee, EmployeeId};

pub fn get_employee(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee" / Uuid)
        .and(warp::get())
        .and(with_db(db))
        .and_then(functions::ge)
}

pub fn get_employees(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee")
        .and(warp::get())
        .and(with_db(db))
        .and_then(functions::ges)
}

pub fn post_employees(
    db: Arc<Client>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employee")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(functions::pe)
}

fn with_db(db: Arc<Client>) -> impl Filter<Extract = (Arc<Client>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&db))
}

// #[post("/employee")]
// pub async fn post_employees(
//     state: web::Data<AppState>,
//     employees: web::Json<Vec<Employee>>,
// ) -> HttpResponse {
//     match functions::pe(&state, &employees).await {
//         Ok(x) => HttpResponse::Ok().json(x),
//         Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
//     }
// }
