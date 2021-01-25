use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::AppState;

mod data_structures;
mod functions;

use data_structures::*;

#[get("/employee/{id}")]
pub async fn get_employee(
    state: web::Data<AppState>,
    web::Path(id): web::Path<Uuid>,
) -> HttpResponse {
    match functions::ge(&state, &id).await {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[get("/employee")]
pub async fn get_employees(state: web::Data<AppState>) -> HttpResponse {
    match functions::ges(&state).await {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[post("/employee")]
pub async fn post_employees(
    state: web::Data<AppState>,
    employees: web::Json<Vec<Employee>>,
) -> HttpResponse {
    match functions::pe(&state, &employees).await {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
