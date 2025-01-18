use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::{db, utils, AppState};

#[get("/transactions")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let transactions = db::transactions::get_all_of_user(&db, user_id).await;

    HttpResponse::Ok().json(transactions)
}

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub category_id: u64,
    pub r#type: String,
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
}

#[post("/transactions")]
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateTransactionRequest>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let Some(category) = db::categories::get(&db, data.category_id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Category not found"
        }));
    };

    if category.user_id != user_id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    let transaction = db::transactions::create(&db, user_id, &data).await;

    HttpResponse::Created().json(transaction)
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
