use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::{db, utils, AppState};

#[get("/categories")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let categories = db::categories::get_all_of_user(&db, user_id).await;

    HttpResponse::Ok().json(categories)
}

#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[post("/categories")]
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateCategoryRequest>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let category = db::categories::create(&db, user_id, &data).await;

    HttpResponse::Ok().json(category)
}

#[get("/categories/{id}")]
pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<u64>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let category = db::categories::get(&db, id.into_inner()).await;

    if category.user_id != user_id {
        return HttpResponse::Unauthorized()
            .json(json!({"status": "error", "message": "Unauthorized"}));
    }

    HttpResponse::Ok().json(category)
}

#[derive(Deserialize, Debug)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[put("/categories/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<u64>,
    data: web::Json<UpdateCategoryRequest>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let category = db::categories::get(&db, id.into_inner()).await;

    if category.user_id != user_id {
        return HttpResponse::Unauthorized()
            .json(json!({"status": "error", "message": "Unauthorized"}));
    }

    db::categories::update(&db, category.id, &data).await;

    let category = db::categories::get(&db, category.id).await;

    HttpResponse::Ok().json(category)
}

#[delete("/categories/{id}")]
pub async fn destroy(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<u64>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let category = db::categories::get(&db, id.into_inner()).await;

    if category.user_id != user_id {
        return HttpResponse::Unauthorized()
            .json(json!({"status": "error", "message": "Unauthorized"}));
    }

    db::categories::destroy(&db, category.id).await;

    HttpResponse::Ok().json(json!({"status": "success"}))
}
