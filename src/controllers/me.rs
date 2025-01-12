use actix_web::{get, post, Responder};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    "Profile"
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    "Update Profile"
}
