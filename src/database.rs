use sqlx::SqlitePool;
use chrono::{DateTime, Utc};

pub struct Measurement {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub temperature: f32,
    pub humidity: f32,
    pub light_intensity: f32,
}

pub struct ControlSettings {
    pub id: i64,
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
) -> Result<(), sqlx::Error> {
    let timestamp = Utc::now();
    sqlx::query(
        "INSERT INTO measurements (timestamp, temperature, humidity, light_intensity)
        VALUES (?, ?, ?, ?)"
    )
    .bind(timestamp)
    .bind(temperature)
    .bind(humidity)
    .bind(light_intensity)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_latest_measurements(pool: &SqlitePool) -> Result<Measurement, sqlx::Error> {
    let row = sqlx::query_as!(
        Measurement,
        "SELECT id, timestamp as \"timestamp: DateTime<Utc>\", temperature, humidity, light_intensity 
        FROM measurements
        ORDER BY timestamp DESC
        LIMIT 1"
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_control_settings(pool: &SqlitePool) -> Result<ControlSettings, sqlx::Error> {
    let row = sqlx::query_as!(
        ControlSettings,
        "SELECT id, target_temperature, target_humidity, target_light_intensity
        FROM control_settings
        ORDER BY id DESC
        LIMIT 1"
    )
    .fetch_one(pool)
    .await
    .or_else(|_| async {
        // Si aucun paramètre de contrôle n'est trouvé, insérez des valeurs par défaut
        sqlx::query!(
            "INSERT INTO control_settings (target_temperature, target_humidity, target_light_intensity)
            VALUES (?, ?, ?)",
            20.0,
            50.0,
            500.0
        )
        .execute(pool)
        .await?;

        // Récupérez les paramètres de contrôle insérés
        sqlx::query_as!(
            ControlSettings,
            "SELECT id, target_temperature, target_humidity, target_light_intensity
            FROM control_settings
            ORDER BY id DESC
            LIMIT 1"
        )
        .fetch_one(pool)
        .await
    })?;

    Ok(row)
}

pub async fn update_control_settings(
    pool: &SqlitePool,
    target_temperature: Option<f32>,
    target_humidity: Option<f32>,
    target_light_intensity: Option<f32>,
) -> Result<(), sqlx::Error> {
    let current_settings = get_control_settings(pool).await?;

    let new_target_temperature = target_temperature.unwrap_or(current_settings.target_temperature);
    let new_target_humidity = target_humidity.unwrap_or(current_settings.target_humidity);
    let new_target_light_intensity = target_light_intensity.unwrap_or(current_settings.target_light_intensity);

    sqlx::query!(
        "INSERT INTO control_settings (target_temperature, target_humidity, target_light_intensity)
        VALUES (?, ?, ?)",
        new_target_temperature,
        new_target_humidity,
        new_target_light_intensity
    )
    .execute(pool)
    .await?;

    Ok(())
}
