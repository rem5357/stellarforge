use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::env;

pub type DbPool = Pool;

pub async fn init_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = database_url.parse::<tokio_postgres::Config>()?;

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = Manager::from_config(config, NoTls, mgr_config);

    let pool = Pool::builder(mgr)
        .max_size(16)
        .build()
        .expect("Failed to create connection pool");

    // Test the connection
    let client = pool.get().await?;
    client.execute("SELECT 1", &[]).await?;
    log::info!("Database connection test successful");

    Ok(pool)
}
