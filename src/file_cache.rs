use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use lru::LruCache;

#[derive(Debug, Clone)]
pub struct FileCache {
    pub lru_data: Arc<Mutex<LruCache<String, String>>>,
}

impl FileCache {
    #[allow(clippy::await_holding_lock)]
    pub async fn cached_get(&self, path: String) -> Option<String> {
        let mut lock = self.lru_data.lock().unwrap();
        let data = lock.get(&path);
        if data.is_none() {
            drop(lock);
            let file = tokio::fs::read_to_string(path.clone()).await;
            if file.is_err() {
                return None;
            }
            let file = file.unwrap();
            let mut lock = self.lru_data.lock().unwrap();
            lock.put(path, file.clone());
            return Some(file);
        }
        Some(data.unwrap().clone())
    }
    pub fn new(cap: usize) -> Self {
        FileCache {
            lru_data: Arc::new(Mutex::new(LruCache::new(cap))),
        }
    }
}
