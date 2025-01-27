use actix_web::web;

use crate::handlers::user_handler::{create_user, get_user, health_check};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/health", web::get().to(health_check)),
    );
}