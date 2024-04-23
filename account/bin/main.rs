use account::router;
use anyhow::{Context, Result};
use axum::{routing::get, IntoResponse, Router};

const BASE_URL: &str = "127.0.0.1";
const PORT: &str = "3000";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("{BASE_URL}:{PORT}"))
        .await
        .with_context(|| "Failed to bind TcpListener")?;

    let router = axum::Router::new()

    axum::serve(listener, router)
        .await
        .with_context(|| "Serving failed.")?;

    Ok(())
}
