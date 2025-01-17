use serde::Serialize;

use crate::controllers::categories::CreateCategoryRequest;

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

pub async fn get(db: &sqlx::MySqlPool, id: u64) -> Category {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = ?", id)
        .fetch_one(db)
        .await
        .unwrap()
}

pub async fn create(
    db: &sqlx::MySqlPool,
    user_id: u64,
    category: &CreateCategoryRequest,
) -> Category {
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
