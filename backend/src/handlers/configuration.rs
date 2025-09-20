use actix_web::{HttpResponse, Responder};
use chrono::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: DateTime<Utc>,
}

pub async fn health_check() -> impl Responder {
    let resp = HealthResponse {
        status: "ok",
        timestamp: Utc::now(),
    };
    HttpResponse::Ok().json(resp)
}

#[derive(Serialize)]
struct ApiRoute {
    method: &'static str,
    path: &'static str,
}

pub async fn get_api_routes() -> impl Responder {
    let routes = vec![
        ApiRoute { method: "GET", path: "/health" },
        ApiRoute { method: "GET", path: "/api/routes" },
    ];
    HttpResponse::Ok().json(routes)
}
