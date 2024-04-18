use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};

mod sensors;
mod database;
mod control;
mod api;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    database_url: String,
    server_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = toml::from_str(&std::fs::read_to_string("config.toml")?)?;
    let pool = SqlitePool::connect(&config.database_url).await?;
    let pool = Arc::new(pool);

    let sensors_manager = Arc::new(Mutex::new(sensors::Manager::new()));
    let control_manager = Arc::new(Mutex::new(control::Manager::new()));

    let _ = tokio::join!(
        sensors::run(sensors_manager.clone(), pool.clone()),
        control::run(control_manager.clone(), pool.clone()),
        api::run(config.server_port, sensors_manager, control_manager, pool)
    );

    Ok(())
}
