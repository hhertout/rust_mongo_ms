use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::controllers::MessageResponse;
use crate::router::AppState;
use crate::repository::post_repository::{NewPost, Post, PostRepository};

#[derive(Serialize, Deserialize)]
pub struct CreatePostResponse {
    pub id: String,
}

pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, (StatusCode, Json<MessageResponse>)> {
    match PostRepository::new(&state.repository).get_all().await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: err.to_string()
            })
        ))
    }
}

pub async fn create_post(State(state): State<AppState>, Json(post): Json<NewPost>) -> Result<Json<CreatePostResponse>, (StatusCode, Json<MessageResponse>)> {
    let service = PostRepository::new(&state.repository);

    match service.insert_one(post).await {
        Ok(id) => Ok(Json(CreatePostResponse { id })),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: err.to_string()
            })
        ))
    }
}