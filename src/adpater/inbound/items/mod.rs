use axum::{Router, routing::get};

// use crate::application::users::get_users_by_id;

// use super::super::application::users::get_users_by_id;

pub fn item_route() -> Router {
    Router::new().route("/items", get(|| async { "item router" }))
}
