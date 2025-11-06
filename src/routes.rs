use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::router::AIRouter;

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub provider: Option<String>, // "openai" | "gemini"
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub provider: String,
    pub output: String,
    pub cached: bool,
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "AI Gateway is healthy")
}

pub async fn generate(
    State(router): State<Arc<AIRouter>>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, crate::error::GatewayError>{
    tracing::info!("Request: prompt length={}, provider={:?}", payload.prompt.len(), payload.provider);

    let (output, provider, cached) = match payload.provider {
        Some(ref provider_name) => {
            router.generate_with_provider(&payload.prompt, provider_name).await?
        }
        None => {
            router.generate(&payload.prompt).await?
        }
    };

    tracing::info!("Response: provider={}, cached={}, length={}",
        provider, cached, output.len());

    Ok(Json(GenerateResponse {
        provider,
        output,
        cached
    }))
}
