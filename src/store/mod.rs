use std::collections::HashMap;

use tokio::sync::RwLock;

use async_trait::async_trait;

mod pg;
pub use pg::new_pg_store;

#[derive(Debug)]
pub enum StoreError {
    EntryNotFound,
    ConnectionError(String),
}

pub type StoreResult<T> = Result<T, StoreError>;

#[async_trait]
pub trait Store: Send + Sync {
    async fn get(&self, key: &str) -> StoreResult<String>;
    async fn set(&self, key: &str, value: &str) -> StoreResult<()>;
}

pub struct MemoryStore {
    data: RwLock<HashMap<String, String>>,
}

pub fn new_memory_store() -> MemoryStore {
    MemoryStore {
        data: RwLock::new(HashMap::new()),
    }
}

#[async_trait]
impl Store for MemoryStore {
    async fn get(&self, key: &str) -> StoreResult<String> {
        let map = self.data.read().await;
        match map.get(key).cloned() {
            Some(val) => Ok(val),
            None => Err(StoreError::EntryNotFound),
        }
    }

    async fn set(&self, key: &str, value: &str) -> StoreResult<()> {
        let mut map = self.data.write().await;
        map.insert(key.to_string(), value.to_string());
        Ok(())
    }
}
