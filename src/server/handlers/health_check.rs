use actix_web::{HttpRequest, HttpResponse, Responder, get};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
