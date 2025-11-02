use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "stellarforge-backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
