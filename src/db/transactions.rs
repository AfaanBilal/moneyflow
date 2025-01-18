use serde::Serialize;

#[derive(Serialize)]
pub struct Transaction {
    id: u64,
    user_id: u64,
    category_id: u64,
    r#type: String,
    amount: u64,
    memo: String,
    description: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

pub async fn get_all_of_user(db: &sqlx::MySqlPool, user_id: u64) -> Vec<Transaction> {
    sqlx::query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE user_id = ?",
        user_id
    )
    .fetch_all(db)
    .await
    .unwrap()
}
