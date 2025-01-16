use bcrypt::{hash, DEFAULT_COST};
use serde::Serialize;
use sqlx::types::chrono;

use crate::controllers::{auth::SignUpRequest, me::UpdateProfileRequest};

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

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    #[serde(skip_serializing)]
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

pub async fn get_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn update(db: &sqlx::MySqlPool, id: u64, user: &UpdateProfileRequest) {
    sqlx::query!(
        "UPDATE users SET firstname = ?, lastname = ? WHERE id = ?",
        &user.firstname,
        &user.lastname,
        id
    )
    .execute(db)
    .await
    .unwrap();
}
