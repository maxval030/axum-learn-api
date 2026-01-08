use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};

use crate::{adpater::outbound::repository::users::repo_get_user_by_id, domain::users::User};

pub async fn handle(Path(id): Path<i32>) -> impl IntoResponse {
    let result = handle_usecase(id).await;

    let Ok(result) = result else {
        return (
            StatusCode::NOT_FOUND,
            Json(format!("User with id {} not found", id)),
        )
            .into_response();
    };

    (StatusCode::OK, Json(result)).into_response()
}

pub async fn handle_usecase(id: i32) -> Result<User, String> {
    let result = repo_get_user_by_id(id).await;

    result
}
