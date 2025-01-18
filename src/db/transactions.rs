/**
 * 💵 MoneyFlow
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/moneyflow
 */
use serde::Serialize;

use crate::controllers::transactions::{CreateTransactionRequest, UpdateTransactionRequest};

#[derive(Serialize)]
pub struct Transaction {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub r#type: String,
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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

pub async fn get_all_of_category(db: &sqlx::MySqlPool, category_id: u64) -> Vec<Transaction> {
    sqlx::query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE category_id = ?",
        category_id
    )
    .fetch_all(db)
    .await
    .unwrap()
}

pub async fn get(db: &sqlx::MySqlPool, id: u64) -> Option<Transaction> {
    sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE id = ?", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn create(
    db: &sqlx::MySqlPool,
    user_id: u64,
    transaction: &CreateTransactionRequest,
) -> Transaction {
    let r= sqlx::query!("INSERT INTO transactions (user_id, category_id, type, amount, memo, description) VALUES (?, ?, ?, ?, ?, ?)", user_id, transaction.category_id, transaction.r#type, transaction.amount, transaction.memo, transaction.description)
        .execute(db)
        .await
        .unwrap();

    get(db, r.last_insert_id()).await.unwrap()
}

pub async fn update(db: &sqlx::MySqlPool, id: u64, transaction: &UpdateTransactionRequest) {
    sqlx::query!(
        "UPDATE transactions SET memo = ?, description = ? WHERE id = ?",
        &transaction.memo,
        &transaction.description,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn destroy(db: &sqlx::MySqlPool, id: u64) {
    sqlx::query!("DELETE FROM transactions WHERE id = ?", id)
        .execute(db)
        .await
        .unwrap();
}
