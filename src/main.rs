mod handlers;
mod store;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let store = Arc::new(store::new());
    let h = handlers::new(store);
    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .route("/url", post(handlers::short_url))
        .route("/url/{short_url}", get(handlers::get_url))
        .with_state(h);

    let listener = tokio::net::TcpListener::bind("localhost:8888")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
