// Fichier : database.rs

use sqlx::SqlitePool;
use chrono::{DateTime, Utc};

pub struct Measurement {
    pub temperature: f32,
    pub humidity: f32,
    pub light_intensity: f32,
}

pub struct ControlSettings {
    pub target_temperature: f32,
    pub target_humidity: f32,
    pub target_light_intensity: f32,
}

pub async fn initialize_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS measurements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            temperature REAL NOT NULL,
            humidity REAL NOT NULL,
            light_intensity REAL NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS control_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            target_temperature REAL NOT NULL,
            target_humidity REAL NOT NULL,
            target_light_intensity REAL NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_measurement(
    pool: &SqlitePool,
    temperature: f32,
    humidity: f32,
    light_intensity: f32,
    timestamp: &DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO measurements (timestamp, temperature, humidity, light_intensity)
        VALUES (?, ?, ?, ?)"
    )
    .bind(timestamp.to_rfc3339())
    .bind(temperature)
    .bind(humidity)
    .bind(light_intensity)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_latest_measurements(pool: &SqlitePool) -> Result<Measurement, sqlx::Error> {
    let row = sqlx::query_as::<_, Measurement>(
        "SELECT temperature, humidity, light_intensity
        FROM measurements
        ORDER BY timestamp DESC
        LIMIT 1"
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_control_settings(pool: &SqlitePool) -> Result<ControlSettings, sqlx::Error> {
    let row = sqlx::query_as::<_, ControlSettings>(
        "SELECT target_temperature, target_humidity, target_light_intensity
        FROM control_settings
        ORDER BY id DESC
        LIMIT 1"
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_control_settings(
    pool: &SqlitePool,
    target_temperature: f32,
    target_humidity: f32,
    target_light_intensity: f32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO control_settings (target_temperature, target_humidity, target_light_intensity)
        VALUES (?, ?, ?)"
    )
    .bind(target_temperature)
    .bind(target_humidity)
    .bind(target_light_intensity)
    .execute(pool)
    .await?;

    Ok(())
}
