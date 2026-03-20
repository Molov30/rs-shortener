use std::sync::Arc;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use axum_valid::Valid;
use serde::{Deserialize, Serialize};

use rand::RngExt;
use rand::distr::Alphanumeric;
use validator::Validate;

use crate::store::Store;

#[derive(Clone)]
pub struct Handler {
    store: Arc<Store>,
}

pub fn new(store: Arc<Store>) -> Handler {
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
    state.store.set(&key, &url.url).await;
    (
        StatusCode::CREATED,
        Json(ShortUrlResponse {
            url: url.url,
            short_url: key,
        }),
    )
        .into_response()
}

#[derive(Serialize)]
pub struct GetUrlResponse {
    url: String,
}

pub async fn get_url(state: State<Handler>, Path(short_url): Path<String>) -> impl IntoResponse {
    match state.store.get(&short_url).await {
        Some(url) => (StatusCode::OK, Json(GetUrlResponse { url })).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn ping() -> &'static str {
    "pong"
}

fn random_string(len: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
