use crate::handlers::handler;
use crate::AppState;
use axum::{
    routing::{get, post},
    Json, Router,
};

pub fn create_api_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/status",
            get(|| async {
                Json(serde_json::json!({ "status": "success", "message": "API is running" }))
            }),
        )
        .route("/paste", post(handler::create_paste))
        .route("/{shortlink}", get(handler::get_paste_by_shortlink))
        .with_state(state) 
}
