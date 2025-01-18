use actix_web::{delete, get, post, put, Responder};

#[get("/transactions")]
pub async fn index() -> impl Responder {
    "Transactions: List"
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
