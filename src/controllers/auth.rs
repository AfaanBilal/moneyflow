use actix_web::{post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SignUpRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(data: web::Json<SignUpRequest>) -> impl Responder {
    format!("Sign Up: {:?}", data)
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign In"
}
