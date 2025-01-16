use bcrypt::{hash, DEFAULT_COST};
use sqlx::types::chrono;

use crate::controllers::auth::SignUpRequest;

pub async fn has_with_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!("SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap()
        .is_some()
}

pub async fn create(db: &sqlx::MySqlPool, user: &SignUpRequest) -> bool {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    sqlx::query!(
        "INSERT INTO users (`email`, `password`, `firstname`, `lastname`) VALUES (?, ?, ?, ?)",
        &user.email,
        &hashed_password,
        &user.firstname,
        &user.lastname
    )
    .execute(db)
    .await
    .is_ok()
}

pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub balance: u64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_by_email(db: &sqlx::MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap()
}
