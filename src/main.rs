use axum::{
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

mod routes;
mod providers;
mod router;
mod cache;
mod error;

#[derive(Clone)]
struct AppState {
    router: Arc<router::AIRouter>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = Arc::new(providers::openai::OpenAIClient::new(openai_api_key));

    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let gemini_client = Arc::new(providers::gemini::GeminiClient::new(gemini_api_key));

    let mut ai_router = router::AIRouter::new();
    ai_router.add_provider(openai_client);
    ai_router.add_provider(gemini_client);

    let state = AppState {
        router: Arc::new(ai_router),
    };

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/generate", post(routes::generate))
        .with_state(state.router);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server started at {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai() {
        dotenv().ok();
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let client = providers::openai::OpenAIClient::new(api_key);

        let result = client.generate("Say hello in one word").await;
        println!("OpenAI response: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_gemini() {
        dotenv().ok();
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
        let client = providers::gemini::GeminiClient::new(api_key);

        let result = client.generate("Say hello in one word").await;
        println!("Gemini response: {:?}", result);
        assert!(result.is_ok());
    }
}
