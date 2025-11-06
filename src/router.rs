use crate::error::GatewayError;
use crate::providers::AIProvider;
use std::sync::Arc;
use crate::cache::Cache;

pub struct AIRouter {
    providers: Vec<Arc<dyn AIProvider>>,
    cache: Cache,
}

impl AIRouter {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            cache: Cache::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Arc<dyn AIProvider>) {
        self.providers.push(provider);
    }

    pub async fn generate(&self, prompt: &str) -> Result<(String, String, bool), GatewayError> {
        if let Some((response, provider_name)) = self.cache.get(prompt) {
            return Ok((response.clone(), provider_name.clone(), true));
        }
        let mut last_error = None;
        for provider in &self.providers {
            tracing::info!("Trying provider: {}", provider.name());
            match provider.generate(prompt).await {
                Ok(response) => {
                    let provider_name = provider.name().to_string();
                    tracing::info!("Provider {} succeeded", provider_name.clone());
                    self.cache.set(prompt.to_string(), response.clone(), provider_name.clone());
                    return Ok(( response, provider_name, false));
                }
                Err(err) => {
                    tracing::warn!("Provider {} failed: {}", provider.name(), err);
                    last_error = Some(err);
                }
            }
        }
        Err(last_error.unwrap_or(GatewayError::AllProvidersFailed))
    }
}
