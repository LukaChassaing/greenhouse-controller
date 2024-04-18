// Fichier : control.rs

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use sqlx::SqlitePool;
use crate::database;
use rppal::gpio::Gpio;

pub struct Manager {
    ventilation_system: VentilationSystem,
    heating_system: HeatingSystem,
    lighting_system: LightingSystem,
}

impl Manager {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            ventilation_system: VentilationSystem::new(1, "Ventilation System", gpio.clone(), 23),
            heating_system: HeatingSystem::new(2, "Heating System", gpio.clone(), 24),
            lighting_system: LightingSystem::new(3, "Lighting System", gpio, 25),
        }
    }

    // ... (Les méthodes de contrôle restent inchangées) ...
}

pub struct VentilationSystem {
    id: i32,
    name: String,
    relay: rppal::gpio::OutputPin,
}

impl VentilationSystem {
    pub fn new(id: i32, name: &str, gpio: Gpio, pin: u8) -> Self {
        let relay = gpio.get(pin).unwrap().into_output();
        Self {
            id,
            name: name.to_string(),
            relay,
        }
    }

    pub async fn turn_on(&mut self) {
        self.relay.set_high();
        println!("Ventilation system turned on");
    }

    pub async fn turn_off(&mut self) {
        self.relay.set_low();
        println!("Ventilation system turned off");
    }
}

pub struct HeatingSystem {
    id: i32,
    name: String,
    relay: rppal::gpio::OutputPin,
}

impl HeatingSystem {
    pub fn new(id: i32, name: &str, gpio: Gpio, pin: u8) -> Self {
        let relay = gpio.get(pin).unwrap().into_output();
        Self {
            id,
            name: name.to_string(),
            relay,
        }
    }

    pub async fn turn_on(&mut self) {
        self.relay.set_high();
        println!("Heating system turned on");
    }

    pub async fn turn_off(&mut self) {
        self.relay.set_low();
        println!("Heating system turned off");
    }
}

pub struct LightingSystem {
    id: i32,
    name: String,
    relay: rppal::gpio::OutputPin,
}

impl LightingSystem {
    pub fn new(id: i32, name: &str, gpio: Gpio, pin: u8) -> Self {
        let relay = gpio.get(pin).unwrap().into_output();
        Self {
            id,
            name: name.to_string(),
            relay,
        }
    }

    pub async fn turn_on(&mut self) {
        self.relay.set_high();
        println!("Lighting system turned on");
    }

    pub async fn turn_off(&mut self) {
        self.relay.set_low();
        println!("Lighting system turned off");
    }
}

pub async fn run(manager: Arc<Mutex<Manager>>, pool: Arc<SqlitePool>) {
    let mut interval = time::interval(Duration::from_secs(60)); // Intervalle de contrôle (60 secondes)
    loop {
        interval.tick().await;
        let control_settings = database::get_control_settings(&pool).await.unwrap();
        let latest_measurements = database::get_latest_measurements(&pool).await.unwrap();

        let temperature = latest_measurements.temperature;
        let humidity = latest_measurements.humidity;
        let light_intensity = latest_measurements.light_intensity;

        let mut manager_locked = manager.lock().await;

        manager_locked.control_ventilation(temperature, humidity).await;
        manager_locked.control_heating(temperature).await;
        manager_locked.control_lighting(light_intensity).await;
    }
}
