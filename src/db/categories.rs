use serde::Serialize;

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
