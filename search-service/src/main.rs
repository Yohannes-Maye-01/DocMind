use anyhow::Result;
use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

mod api;
mod index;
mod parser;
mod search;
mod typesense;

#[tokio::main]
async fn main() -> Result<()> {
    // Structured JSON logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(
            "search_service=info".parse()?,
        ))
        .json()
        .init();

    dotenvy::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".into())
        .parse()?;

    let app: Router = api::router();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    info!("DocMind search-service listening on :{port}");

    serve(listener, app).await?;
    Ok(())
}
