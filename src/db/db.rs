use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::env;
use std::time::Duration;

/// Connects to the database using the connection options specified in the environment variables.
///
/// # Returns
///
/// A `DatabaseConnection` instance.
pub async fn connect() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let mut opt = ConnectOptions::new(db_url);
    opt.connect_timeout(Duration::from_secs(30));

    Database::connect(opt)
        .await
        .expect("Failed to connect to database")
}
