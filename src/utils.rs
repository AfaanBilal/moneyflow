use actix_web::{HttpMessage, HttpRequest};

pub fn get_user_id(req: &HttpRequest) -> u64 {
    let ext = req.extensions();

    ext.get::<u64>().unwrap().to_owned()
}
