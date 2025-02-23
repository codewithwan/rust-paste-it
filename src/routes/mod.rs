pub mod api;
pub mod web;

use axum::Router;
use crate::AppState;

pub fn create_routes(state: AppState) -> Router {
    let api_state = state.clone();
    Router::new()
        .nest("/api", api::create_api_routes(api_state)) // Nest API routes under /api
        .merge(web::create_web_routes(state)) // Merge web routes
}

