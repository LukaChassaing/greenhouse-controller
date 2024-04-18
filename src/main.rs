// Fichier : main.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;
use std::env;

mod sensors;
mod control;
mod database;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port: u16 = env::var("API_PORT").expect("API_PORT must be set").parse()?;

    let pool = SqlitePool::connect(&database_url).await?;
    let pool = Arc::new(pool);

    database::initialize_database(&pool).await?;

    let sensors_manager = Arc::new(Mutex::new(sensors::Manager::new()));
    let control_manager = Arc::new(Mutex::new(control::Manager::new()));

    let sensors_task = tokio::spawn(sensors::run(sensors_manager.clone(), pool.clone()));
    let control_task = tokio::spawn(control::run(control_manager.clone(), pool.clone()));
    let api_task = tokio::spawn(api::run(port, sensors_manager, control_manager, pool));

    tokio::select! {
        _ = sensors_task => {},
        _ = control_task => {},
        _ = api_task => {},
    }

    Ok(())
}
