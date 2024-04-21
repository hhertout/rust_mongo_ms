use axum::Json;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    message: String,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        message: String::from("I'm alive")
    })
}