use lru::LruCache;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct FileCache {
  pub lru_data: Arc<Mutex<LruCache<String, String>>>,
}

unsafe impl Sync for FileCache {}
unsafe impl Send for FileCache {}

impl FileCache {
  #[allow(clippy::await_holding_lock)]
  pub async fn cached_get(&self, path: String) -> Option<String> {
    let mut lock = self.lru_data.lock().await;
    let data = lock.get(&path);
    if data.is_none() {
      log::info!("cache miss for {}", &path);
      drop(lock);
      let file = tokio::fs::read_to_string(path.clone()).await;
      if file.is_err() {
        return None;
      }
      let file = file.unwrap();
      let mut lock = self.lru_data.lock().await;
      lock.put(path, file.clone());
      return Some(file);
    }
    log::info!("cache hit for {}", &path);
    Some(data.unwrap().clone())
  }
  pub fn new(cap: usize) -> Self {
    FileCache {
      lru_data: Arc::new(Mutex::new(LruCache::new(cap))),
    }
  }
}
