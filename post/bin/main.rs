use anyhow::{Context, Result};
use axum::{
    extract::Path,
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::Serialize;

const BASE_URL: &str = "127.0.0.1";
const PORT: &str = "3001";

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(get_health));

    let listener = tokio::net::TcpListener::bind(format!("{BASE_URL}:{PORT}"))
        .await
        .with_context(|| "Failed to bind TcpListener")?;

    axum::serve(listener, app)
        .await
        .with_context(|| "Serving failed.")?;

    Ok(())
}

#[derive(Serialize)]
struct Health {
    status: String,
}

impl IntoResponse for Health {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

async fn get_health() -> impl IntoResponse {
    Health {
        status: String::from("Working"),
    };
}
