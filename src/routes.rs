use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

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
    Json(payload): Json<GenerateRequest>,
) -> impl IntoResponse {
    let response = GenerateResponse {
        provider: "openai".to_string(),
        output: "Generated text".to_string(),
    };

    (StatusCode::OK, Json(response))
}
