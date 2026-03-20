use std::collections::HashMap;

use tokio::sync::RwLock;

pub struct Store {
    data: RwLock<HashMap<String, String>>,
}

pub fn new() -> Store {
    Store {
        data: RwLock::new(HashMap::new()),
    }
}

impl Store {
    pub async fn get(&self, key: &str) -> Option<String> {
        let map = self.data.read().await;
        map.get(key).cloned()
    }

    pub async fn set(&self, key: &str, value: &str) {
        let mut map = self.data.write().await;
        map.insert(key.to_string(), value.to_string());
    }
}
