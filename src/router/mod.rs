use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::config;
use crate::config::database;
use crate::controllers;

#[derive(Clone)]
pub struct AppState {
   pub mongo_client: Arc<mongodb::Client>,
}

pub async fn serve() -> Router {
    let state = AppState {
        mongo_client: Arc::from(database::connect().await)
    };

    let api = Router::new();

    Router::new()
        .route("/health", get(controllers::health_check))
        .nest("/api/v1", api)
        .layer(config::cors::cors_layer())
        .with_state(state)
}
