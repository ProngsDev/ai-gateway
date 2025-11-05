use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Cache {
    store: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

struct CacheEntry {
    response: String,
    provider: String,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, prompt: &str) -> Option<(String, String)> {
        let store = self.store.lock().unwrap();
        store.get(prompt).map(|entry| {
            tracing::info!("Cache hit for prompt");
            (entry.response.clone(), entry.provider.clone())
        })
    }

    pub fn set(&self, prompt: String, response: String, provider: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(prompt, CacheEntry {
            response,
            provider,
        });
    }
}
