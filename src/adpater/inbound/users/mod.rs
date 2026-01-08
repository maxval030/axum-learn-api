use axum::{Router, routing::get};

use crate::application::users::{get_all_user, get_users_by_id};

// use super::super::application::users::get_users_by_id;

pub fn user_route() -> Router {
    Router::new()
        .route("/user", get(|| async { "user router" }))
        .route("/user-all", get(get_all_user::handle))
        .route("/user-id/{id}", get(get_users_by_id::handle))
        .route("/user-asdf/{id}", get(|id| get_users_by_id::handle(id)))
}
