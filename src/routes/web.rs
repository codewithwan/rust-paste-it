use crate::views::index; 
use crate::AppState;
use axum::{
    routing::{get, any},
    Router,
};

pub fn create_web_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/api-docs", get(index::api_docs))
        .fallback(any(index::not_found))
        .with_state(state)
}
