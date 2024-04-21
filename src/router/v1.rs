use axum::Router;
use axum::routing::{get, post};
use crate::controllers;
use crate::router::AppState;

pub(crate) fn router()-> Router<AppState> {
    Router::new()
        .route("/posts", get(controllers::post_controller::get_posts))
        .route("/post", post(controllers::post_controller::create_post))
}