// Fichier : api.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::SqlitePool;
use warp::{Filter, Rejection, Reply};
use crate::sensors;
use crate::control;
use crate::database::{Measurement, ControlSettings};

type Result<T> = std::result::Result<T, Rejection>;

pub async fn run(
    port: u16,
    sensors_manager: Arc<Mutex<sensors::Manager>>,
    control_manager: Arc<Mutex<control::Manager>>,
    pool: Arc<SqlitePool>,
) {
    let sensors_manager_filter = warp::any().map(move || sensors_manager.clone());
    let control_manager_filter = warp::any().map(move || control_manager.clone());
    let pool_filter = warp::any().map(move || pool.clone());

    let measurements_route = warp::path("measurements")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(get_latest_measurements);

    let control_settings_route = warp::path("control_settings")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(get_control_settings)
        .or(warp::path("control_settings")
            .and(warp::put())
            .and(warp::body::json())
            .and(pool_filter.clone())
            .and_then(update_control_settings));

    let routes = measurements_route
        .or(control_settings_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}

async fn get_latest_measurements(pool: Arc<SqlitePool>) -> Result<impl Reply> {
    match crate::database::get_latest_measurements(&pool).await {
        Ok(measurement) => Ok(warp::reply::json(&measurement)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn get_control_settings(pool: Arc<SqlitePool>) -> Result<impl Reply> {
    match crate::database::get_control_settings(&pool).await {
        Ok(settings) => Ok(warp::reply::json(&settings)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn update_control_settings(settings: ControlSettings, pool: Arc<SqlitePool>) -> Result<impl Reply> {
    match crate::database::update_control_settings(
        &pool,
        settings.target_temperature,
        settings.target_humidity,
        settings.target_light_intensity,
    ).await {
        Ok(_) => Ok(warp::reply::with_status("Control settings updated", warp::http::StatusCode::OK)),
        Err(_) => Err(warp::reject::not_found()),
    }
}
