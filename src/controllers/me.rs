use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{db, utils, AppState};

#[get("/me")]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user = db::user::get_by_id(&db, utils::get_user_id(&req)).await.unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    "Update Profile"
}
