/**
 * 💵 MoneyFlow
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/moneyflow
 */
use serde::Serialize;

use crate::controllers::categories::{CreateCategoryRequest, UpdateCategoryRequest};

#[derive(Serialize)]
pub struct Category {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_all_of_user(db: &sqlx::MySqlPool, user_id: u64) -> Vec<Category> {
    sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE user_id = ?",
        user_id
    )
    .fetch_all(db)
    .await
    .unwrap()
}

pub async fn get(db: &sqlx::MySqlPool, id: u64) -> Option<Category> {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = ?", id)
        .fetch_one(db)
        .await
        .ok()
}

pub async fn create(
    db: &sqlx::MySqlPool,
    user_id: u64,
    category: &CreateCategoryRequest,
) -> Option<Category> {
    let r = sqlx::query!(
        "INSERT INTO categories (user_id, name, description) VALUES (?, ?, ?)",
        user_id,
        &category.name,
        &category.description
    )
    .execute(db)
    .await
    .unwrap();

    get(db, r.last_insert_id()).await
}

pub async fn update(db: &sqlx::MySqlPool, id: u64, category: &UpdateCategoryRequest) {
    sqlx::query!(
        "UPDATE categories SET name = ?, description = ? WHERE id = ?",
        &category.name,
        &category.description,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn destroy(db: &sqlx::MySqlPool, id: u64) {
    sqlx::query!("DELETE FROM categories WHERE id = ?", id)
        .execute(db)
        .await
        .unwrap();
}

pub async fn update_balance(db: &sqlx::MySqlPool, id: u64, balance: u64) {
    sqlx::query!(
        "UPDATE categories SET balance = ? WHERE id = ?",
        balance,
        id
    )
    .execute(db)
    .await
    .unwrap();
}
