use std::sync::Arc;

use warp::Filter;

mod auth;
mod db;
mod error;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = db::init().await?;

    let route = health()
        .or(routes::get_employee(Arc::clone(&db)))
        .or(routes::get_self(Arc::clone(&db)))
        .or(routes::get_employees(Arc::clone(&db)))
        .or(routes::post_employees(Arc::clone(&db)))
        .or(routes::login(Arc::clone(&db)));

    warp::serve(route).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}

fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health").and(warp::get()).map(warp::reply)
}
