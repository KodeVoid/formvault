use actix_web::{HttpRequest, HttpResponse, Responder};
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
    url: String,
    description: Option<&'static str>,
}

pub async fn get_api_routes(req: HttpRequest) -> impl Responder {
    let scheme = req.connection_info().scheme().to_string();
    let host = req.connection_info().host().to_string();

    let routes = vec![
        ApiRoute {
            method: "GET",
            url: format!("{}://{}/health_check", scheme, host),
            description: Some("Health check endpoint"),
        },
        ApiRoute {
            method: "GET",
            url: format!("{}://{}/", scheme, host),
            description: Some("API root"),
        },
    ];

    HttpResponse::Ok().json(routes)
}
