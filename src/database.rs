use sqlx::SqlitePool;

pub async fn get_sensors(pool: &SqlitePool) -> Result<Vec<Sensor>, sqlx::Error> {
    sqlx::query_as!(Sensor, "SELECT id, name, type FROM sensors")
        .fetch_all(pool)
        .await
}
