use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseFailed(#[from] serde_json::Error),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("All providers failed")]
    AllProvidersFailed,

    #[error("Missing configuration: {0}")]
    ConfigError(String),
}

impl axum::response::IntoResponse for GatewayError {
    fn into_response(self) -> axum::response::Response {
        let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        let body = format!("Error: {}", self);
        (status, body).into_response()
    }
}
