use actix_web::{HttpResponse, Responder};
use chrono::prelude::*;

pub async fn health_check() -> impl Responder {
    let utc: DateTime<Utc> = Utc::now();

    HttpResponse::Ok().json(utc)
}

pub async fn get_api_routes() -> impl Responder {
    HttpResponse::Ok().json("[]")
}
