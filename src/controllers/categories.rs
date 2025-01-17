use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use crate::{db, utils, AppState};

#[get("/categories")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let categories = db::categories::get_all_of_user(&db, user_id).await;

    HttpResponse::Ok().json(categories)
}

#[post("/categories")]
pub async fn create() -> impl Responder {
    "Categories: Create"
}

#[get("/categories/{id}")]
pub async fn show() -> impl Responder {
    "Categories: Show"
}

#[put("/categories/{id}")]
pub async fn update() -> impl Responder {
    "Categories: Update"
}

#[delete("/categories/{id}")]
pub async fn destroy() -> impl Responder {
    "Categories: Destroy"
}
