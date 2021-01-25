use actix_web::{web, App, HttpServer};

mod routes;
mod state;

use state::AppState;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let state = web::Data::new(AppState::new().await?);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(routes::get_employee)
            .service(routes::get_employees)
            .service(routes::post_employees)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
