use crate::server::handlers::health_check;
use actix_web::web::{self, ServiceConfig};

pub fn int(cfg: &mut ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check::health_check));
}
