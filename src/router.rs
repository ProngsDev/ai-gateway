use crate::error::GatewayError;
use crate::providers::AIProvider;
use std::sync::Arc;

pub struct AIRouter {
    providers: Vec<Arc<dyn AIProvider>>,
}

impl AIRouter {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Arc<dyn AIProvider>) {
        self.providers.push(provider);
    }

    pub async fn generate(&self, prompt: &str) -> Result<(String, String), GatewayError> {
        let mut last_error = None;
        for provider in &self.providers {
            tracing::info!("Trying provider: {}", provider.name());
            match provider.generate(prompt).await {
                Ok(response) => {
                    tracing::info!("Provider {} succeeded", provider.name());
                    return Ok(( response, provider.name().to_string()));
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
