pub mod db;
pub mod entities;
pub mod models;
pub mod handlers;
pub mod routes;
pub mod views; // Update this line

pub use routes::create_routes; // Keep this line

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
