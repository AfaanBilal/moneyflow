use actix_web::{delete, get, post, put, Responder};

#[get("/categories")]
pub async fn index() -> impl Responder {
    "Categories: List"
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
