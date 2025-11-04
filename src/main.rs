use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

mod routes;
mod providers;
mod router;
mod cache;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/generate", post(routes::generate));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server started at {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
