use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::env;

mod api;
mod models;
mod database;
mod generator;

use crate::database::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logger
    env_logger::init();

    log::info!("Starting StellarForge Backend...");

    // Get configuration
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    // Initialize database connection pool
    let pool = init_pool().await.expect("Failed to create database pool");
    log::info!("Database connection pool initialized");

    // Create HTTP server
    log::info!("Starting HTTP server at {}:{}", host, port);

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(api::configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
