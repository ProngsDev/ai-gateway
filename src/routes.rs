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
    pub model: Option<String>, // "openai" | "gemini"
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub provider: String,
    pub output: String,
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "AI Gateway is healthy")
}

pub async fn generate(
    State(router): State<Arc<AIRouter>>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, crate::error::GatewayError>{
    let (output, provider) = router.generate(&payload.prompt).await?;

    Ok(Json(GenerateResponse {
        provider,
        output
    }))
}
