use actix_web::{web, App, HttpServer};
use tokio::sync::Mutex;

mod controllers;
mod db;

struct AppState {
    db: Mutex<sqlx::MySqlPool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let state = web::Data::new(AppState {
        db: Mutex::new(
            sqlx::MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        ),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            .service(controllers::me::get_profile)
            .service(controllers::me::update_profile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
