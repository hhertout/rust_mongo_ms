use axum::Router;
use axum::routing::get;
use crate::config;
use crate::controllers;

#[derive(Clone)]
struct AppState;

pub fn serve() -> Router {
    let state = AppState {};
    let api = Router::new();

    Router::new()
        .route("/health", get(controllers::health_check))
        .nest("/api/v1", api)
        .layer(config::cors::cors_layer())
        .with_state(state)
}
