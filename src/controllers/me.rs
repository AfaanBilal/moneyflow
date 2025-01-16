use actix_web::{get, post, HttpRequest, Responder};

use crate::utils;

#[get("/me")]
pub async fn get_profile(req: HttpRequest) -> impl Responder {
    let user_id = utils::get_user_id(&req);

    format!("Profile of user: {user_id}")
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    "Update Profile"
}
