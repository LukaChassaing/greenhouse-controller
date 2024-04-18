use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::SqlitePool;

pub struct Manager {
   
}

impl Manager {
    pub fn new() -> Self {
        Self {
          
        }
    }

    
}

pub async fn run(manager: Arc<Mutex<Manager>>, pool: Arc<SqlitePool>) {
    
}
