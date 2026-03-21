use std::sync::Arc;
use std::time::Instant;

use axum::extract::Path;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use axum_valid::Valid;
use serde::{Deserialize, Serialize};

use tracing::debug;

use rand::RngExt;
use rand::distr::Alphanumeric;
use validator::Validate;

use crate::store::{self, Store};

#[derive(Clone)]
pub struct Handler {
    store: Arc<dyn Store>,
}

pub fn new(store: Arc<dyn Store>) -> Handler {
    Handler { store: store }
}

#[derive(Deserialize, Validate)]
pub struct ShortUrlRequest {
    #[validate(url)]
    url: String,
}

#[derive(Serialize)]
pub struct ShortUrlResponse {
    url: String,
    short_url: String,
}

pub async fn short_url(
    state: State<Handler>,
    Valid(Json(url)): Valid<Json<ShortUrlRequest>>,
) -> impl IntoResponse {
    let key = random_string(10);
    match state.store.set(&key, &url.url).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(ShortUrlResponse {
                url: url.url,
                short_url: key,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
pub struct GetUrlResponse {
    url: String,
}

pub async fn get_url(state: State<Handler>, Path(short_url): Path<String>) -> impl IntoResponse {
    match state.store.get(&short_url).await {
        Ok(url) => (StatusCode::OK, Json(GetUrlResponse { url })).into_response(),
        Err(e) => match e {
            store::StoreError::EntryNotFound => StatusCode::NOT_FOUND.into_response(),
            store::StoreError::ConnectionError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn log_requests(req: Request, next: Next) -> impl IntoResponse {
    let method = req.method().clone().to_string();
    let uri = req.uri().clone().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    debug!(
        method = method,
        uri = uri,
        status = response.status().as_u16(),
        latency_ms = start.elapsed().as_millis(),
        "log request"
    );

    response
}

fn random_string(len: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
