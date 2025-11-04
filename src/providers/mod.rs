use async_trait::async_trait;
use crate::error::GatewayError;

pub mod openai;
pub mod gemini;

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, GatewayError>;
    fn name(&self) -> String;
}
