use actix_web::{post, web, Responder};
use serde::Deserialize;

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
        return "Email already exists".to_string();
    }

    db::user::create(&db, &data).await;

    "Sign Up Successful".to_string()
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign In"
}
