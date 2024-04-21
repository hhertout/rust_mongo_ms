mod v1;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::config;
use crate::controllers;
use crate::repository::Repository;


#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<Repository>,
}

pub async fn serve() -> Router {
    let state = AppState {
        repository: Arc::from(Repository::new().await)
    };

    Router::new()
        .route("/health", get(controllers::health_check))
        .nest("/api/v1", v1::router())
        .layer(config::cors::cors_layer())
        .with_state(state)
}
