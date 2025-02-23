pub mod db;
pub mod entities;
pub mod models;
pub mod handlers;
pub mod routes;
pub mod views; 

pub use routes::create_routes; 

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
