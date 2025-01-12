use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::{db, AppState};

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().await;

    if db::user::has_with_email(&db, &data.email).await {
        return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "Email already exists."
        }));
    }

    db::user::create(&db, &data).await;

    HttpResponse::Created().json(json!({
        "status": "success",
        "message": "Account created successfully."
    }))
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign In"
}
