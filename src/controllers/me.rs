/**
 * 💵 MoneyFlow
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/moneyflow
 */
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::{db, utils, AppState};

#[get("/me")]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;

    HttpResponse::Ok().json(user)
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub firstname: String,
    pub lastname: String,
}

#[post("/me")]
pub async fn update_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<UpdateProfileRequest>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;

    db::user::update(&db, user.id, &data).await;

    let user = utils::get_authenticated_user(&req, &db).await;

    HttpResponse::Ok().json(user)
}
