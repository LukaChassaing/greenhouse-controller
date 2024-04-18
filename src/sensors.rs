// Fichier : sensors.rs

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use sqlx::SqlitePool;
use crate::database;

pub struct Manager {
    temperature_sensor: TemperatureSensor,
    humidity_sensor: HumiditySensor,
    light_sensor: LightSensor,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            temperature_sensor: TemperatureSensor::new(1, "Temperature Sensor", "DHT22"),
            humidity_sensor: HumiditySensor::new(2, "Humidity Sensor", "DHT22"),
            light_sensor: LightSensor::new(3, "Light Sensor", "BH1750"),
        }
    }

    pub async fn read_sensors(&mut self) -> (f32, f32, f32) {
        let temperature = self.temperature_sensor.read_temperature().await;
        let humidity = self.humidity_sensor.read_humidity().await;
        let light = self.light_sensor.read_light_intensity().await;
        (temperature, humidity, light)
    }
}

pub struct TemperatureSensor {
    id: i32,
    name: String,
    sensor_type: String,
}

impl TemperatureSensor {
    pub fn new(id: i32, name: &str, sensor_type: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            sensor_type: sensor_type.to_string(),
        }
    }

    pub async fn read_temperature(&self) -> f32 {
        // Logique pour lire la température à partir du capteur DHT22
        // Remplacer par le code réel d'interaction avec le capteur
        let temperature = 25.5;
        temperature
    }
}

pub struct HumiditySensor {
    id: i32,
    name: String,
    sensor_type: String,
}

impl HumiditySensor {
    pub fn new(id: i32, name: &str, sensor_type: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            sensor_type: sensor_type.to_string(),
        }
    }

    pub async fn read_humidity(&self) -> f32 {
        // Logique pour lire l'humidité à partir du capteur DHT22
        // Remplacer par le code réel d'interaction avec le capteur
        let humidity = 60.0;
        humidity
    }
}

pub struct LightSensor {
    id: i32,
    name: String,
    sensor_type: String,
}

impl LightSensor {
    pub fn new(id: i32, name: &str, sensor_type: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            sensor_type: sensor_type.to_string(),
        }
    }

    pub async fn read_light_intensity(&self) -> f32 {
        // Logique pour lire l'intensité lumineuse à partir du capteur BH1750
        // Remplacer par le code réel d'interaction avec le capteur
        let light_intensity = 500.0;
        light_intensity
    }
}

pub async fn run(manager: Arc<Mutex<Manager>>, pool: Arc<SqlitePool>) {
    let mut interval = time::interval(Duration::from_secs(60)); // Intervalle de lecture des capteurs (60 secondes)
    loop {
        interval.tick().await;
        let (temperature, humidity, light) = manager.lock().await.read_sensors().await;
        let timestamp = chrono::Utc::now().to_rfc3339();

        // Enregistrer les mesures dans la base de données
        database::insert_temperature_measurement(&pool, temperature, &timestamp).await;
        database::insert_humidity_measurement(&pool, humidity, &timestamp).await;
        database::insert_light_measurement(&pool, light, &timestamp).await;
    }
}
