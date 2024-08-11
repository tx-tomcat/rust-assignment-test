use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::info;

pub struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, (V, Instant)>>>,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_insert_with<F, Fut, E>(&self, key: K, ttl: Duration, f: F) -> Result<V, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<V, E>>,
    {
        let now = Instant::now();

        if let Some((value, expires_at)) = self.data.read().await.get(&key) {
            if *expires_at > now {
                info!(cache_time = ?expires_at.duration_since(now), "Cache hit");
                return Ok(value.clone());
            }
        }

        info!("Cache miss");
        let result = f().await;

        if let Ok(value) = &result {
            self.data
                .write()
                .await
                .insert(key, (value.clone(), now + ttl));
        }
        result
    }
}

pub use cache_macro::cached;
