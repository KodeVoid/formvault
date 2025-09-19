use crate::handlers;
use actix_web::web;

pub fn health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health_check").route(web::get().to(handlers::configuration::health_check)),
    );
}

pub fn api_routes(cfg:&mut web::ServiceConfig){
    cfg.service(
        web::resource("/").route(web::get().to(handlers::configuration::get_api_routes)));
    
}