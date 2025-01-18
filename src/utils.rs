/**
 * 💵 MoneyFlow
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/moneyflow
 */
use actix_web::{HttpMessage, HttpRequest};

use crate::db;

pub fn get_user_id(req: &HttpRequest) -> u64 {
    let ext = req.extensions();

    ext.get::<u64>().unwrap().to_owned()
}

pub async fn get_authenticated_user(req: &HttpRequest, db: &sqlx::MySqlPool) -> db::user::User {
    db::user::get_by_id(db, get_user_id(req)).await.unwrap()
}

pub fn is_credit(t: &str) -> bool {
    t == "CREDIT"
}

pub fn is_debit(t: &str) -> bool {
    t == "DEBIT"
}
