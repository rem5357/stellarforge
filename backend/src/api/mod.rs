pub mod projects;
pub mod health;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(health::configure)
            .configure(projects::configure)
    );
}
