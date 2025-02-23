use dotenv::dotenv;
use tracing::info;
use tracing_subscriber;
use paste_it::db::connect;
use tokio::net::TcpListener;
use paste_it::create_routes; 

use paste_it::AppState;

/// The main entry point of the application.
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = connect().await;
    let state = AppState { db };

    let app = create_routes(state);

    let ip = if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
        == "production"
    {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };
    let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());

    let addr: std::net::SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
    info!("Server running on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
