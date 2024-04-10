use anyhow::{Context, Result};
use axum::{extract::Path, response::IntoResponse, routing::get, Router};

const BASE_URL: &str = "127.0.0.1";
const PORT: &str = "3000";

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(get_index_handler))
        .route("/greet/:greet", get(get_greet_handler));

    let listener = tokio::net::TcpListener::bind(format!("{BASE_URL}:{PORT}"))
        .await
        .with_context(|| "Failed to bind TcpListener")?;

    axum::serve(listener, app)
        .await
        .with_context(|| "Serving failed.")?;

    Ok(())
}

async fn get_index_handler() -> impl IntoResponse {
    "Hello world"
}

async fn get_greet_handler(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!")
}
