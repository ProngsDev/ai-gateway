use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format;
use crate::error::GatewayError;
use super::AIProvider;

pub struct GeminiClient {
    api_key: String,
    http_client: reqwest::Client,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for GeminiClient {
    async fn generate(&self, prompt: &str) -> Result<String, GatewayError> {
        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            self.api_key
        );

        let response = self.http_client.post(&url).json(&request_body).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(GatewayError::ProviderError(format!("Gemini API error: {}", error_text)));
        }

        let gemini_response: GeminiResponse = response.json().await?;
        let text = gemini_response.candidates.first().and_then(|c| c.content.parts.first())

            .map(|p| p.text.clone())
            .ok_or_else(|| GatewayError::ProviderError("No response from Gemini".to_string()))?;

        Ok(text)
    }

    fn name(&self) -> String {
        "Gemini".to_string()
    }
}
