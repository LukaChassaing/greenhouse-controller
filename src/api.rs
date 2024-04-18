use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::SqlitePool;
use warp::Filter;
use crate::sensors;
use crate::control;

pub async fn run(
    port: u16,
    sensors_manager: Arc<Mutex<sensors::Manager>>,
    control_manager: Arc<Mutex<control::Manager>>,
    pool: Arc<SqlitePool>,
) {
    // Définir les routes de l'API web à l'aide de Warp
    let routes = warp::any()
        .and(warp::path("sensors"))
        .and(warp::get())
        .and(with_managers(sensors_manager, control_manager))
        .and(with_pool(pool))
        .and_then(handlers::get_sensors);

    // Démarrer le serveur web
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}

mod handlers {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use sqlx::SqlitePool;
    use warp::reply::json;
    use crate::sensors;
    use crate::control;

    pub async fn get_sensors(
        sensors_manager: Arc<Mutex<sensors::Manager>>,
        control_manager: Arc<Mutex<control::Manager>>,
        pool: Arc<SqlitePool>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Logique pour récupérer les capteurs depuis la base de données
        let sensors = crate::database::get_sensors(&pool).await.map_err(|_| warp::reject())?;
        Ok(json(&sensors))
    }

    // Ajouter ici d'autres gestionnaires pour les différentes routes de l'API
}

fn with_managers(
    sensors_manager: Arc<Mutex<sensors::Manager>>,
    control_manager: Arc<Mutex<control::Manager>>,
) -> impl Filter<Extract = (Arc<Mutex<sensors::Manager>>, Arc<Mutex<control::Manager>>), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || (sensors_manager.clone(), control_manager.clone()))
}

fn with_pool(pool: Arc<SqlitePool>) -> impl Filter<Extract = (Arc<SqlitePool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
