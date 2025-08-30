use actix_web::{HttpResponse, Responder};
use chrono::prelude::*;

pub async fn health_check() -> impl Responder {
    let utc: DateTime<Utc> = Utc::now();

    HttpResponse::Ok().json(utc)
}
