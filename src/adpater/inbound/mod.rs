use axum::Router;

mod items;
mod users;

pub fn all_controller() -> Router {
    Router::new()
        .merge(users::user_route())
        .merge(items::item_route())
}
