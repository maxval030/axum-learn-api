use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{adpater::outbound::repository::users::repo_get_all_users, domain::users::User};

pub async fn handle() -> impl IntoResponse {
    let result = handle_use_case().await;

    (StatusCode::OK, Json(result)).into_response()
}

pub async fn handle_use_case() -> Vec<User> {
    let result = repo_get_all_users(false)
        .await
        .ok()
        .flatten()
        .unwrap_or_default();

    result
}
