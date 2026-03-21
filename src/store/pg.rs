use async_trait::async_trait;
use sqlx::PgPool;

use super::{Store, StoreError, StoreResult};

pub struct PgStore {
    pool: PgPool,
}

pub async fn new_pg_store(pool: PgPool) -> StoreResult<PgStore> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
    Ok(PgStore { pool })
}

#[async_trait]
impl Store for PgStore {
    async fn get(&self, key: &str) -> StoreResult<String> {
        let url: Option<String> = sqlx::query_scalar("SELECT url FROM urls where key = $1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        url.ok_or(StoreError::EntryNotFound)
    }

    async fn set(&self, key: &str, val: &str) -> StoreResult<()> {
        sqlx::query("INSERT INTO urls (key, url) VALUES ($1, $2)")
            .bind(key)
            .bind(val)
            .execute(&self.pool)
            .await
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        return Ok(());
    }
}
