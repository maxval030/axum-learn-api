use axum::{Router, routing::get};
use rust_api_project::adpater::inbound;

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/hello", get(hello_world));
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6520").await.unwrap();

    println!("servier is listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let route = Router::new().route("/test-naja", get(|| async { "route in route" }));
    Router::new().merge(route).merge(inbound::all_controller())
}

// async fn hello_world() -> Router {
//     "hello world".to_string()
// }
