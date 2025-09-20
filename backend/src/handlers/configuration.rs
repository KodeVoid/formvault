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
}

pub async fn get_api_routes(req: HttpRequest) -> impl Responder {
]    let host = req.connection_info().host(); 

    let routes = vec![
        ApiRoute { 
            method: "GET", 
            url: format!("http://{}/health_check", host),
        },
        ApiRoute { 
            method: "GET", 
            url: format!("http://{}/", host),
        },
    ];

    HttpResponse::Ok().json(routes)
}
