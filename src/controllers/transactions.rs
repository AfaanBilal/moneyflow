use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use crate::{db, utils, AppState};

#[get("/transactions")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let transactions = db::transactions::get_all_of_user(&db, user_id).await;

    HttpResponse::Ok().json(transactions)
}

#[post("/transactions")]
pub async fn create() -> impl Responder {
    "Transactions: Create"
}

#[get("/transactions/{id}")]
pub async fn show() -> impl Responder {
    "Transactions: Show"
}

#[put("/transactions/{id}")]
pub async fn update() -> impl Responder {
    "Transactions: Update"
}

#[delete("/transactions/{id}")]
pub async fn destroy() -> impl Responder {
    "Transactions: Destroy"
}
