use actix_web::{post, Responder};

#[post("/auth/sign-up")]
pub async fn sign_up() -> impl Responder {
    "Sign Up"
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign In"
}
