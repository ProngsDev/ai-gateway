use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::GatewayError;
use super::AIProvider;

pub struct OpenAIClient {
    api_key: String,
    http_client: reqwest::Client,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIClient {
    async fn generate(&self, prompt: &str) -> Result<String, GatewayError> {
        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
        };

        let response = self.http_client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_message = response.text().await?;
            return Err(GatewayError::ProviderError(format!("OpenAI API error: {}", error_message)));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let text = openai_response
            .choices
            .first()
            .ok_or_else(|| GatewayError::ProviderError("No choices found".to_string()))?
            .message
            .content.clone();

        Ok(text)
    }

    fn name(&self) -> String {
        "OpenAI".to_string()
    }
}
