mod config;
mod handlers;
mod store;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .json()
        .init();

    let cfg = config::load_config();
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&cfg.db_dsn)
        .await
        .unwrap();

    let store = Arc::new(store::new_pg_store(pool).await.unwrap());
    let h = handlers::new(store);
    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .route("/url", post(handlers::short_url))
        .route("/url/{short_url}", get(handlers::get_url))
        .layer(middleware::from_fn(handlers::log_requests))
        .with_state(h);

    info!(addr = cfg.addr(), "starting app");
    let listener = tokio::net::TcpListener::bind(cfg.addr()).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
