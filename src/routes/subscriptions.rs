use actix_web::{web, HttpResponse};
use serde;

#[allow(dead_code)] // used implicitly by web::Form<FormData> in subscribe
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
