use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::Path;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
    counter: u16,
}

pub async fn run() -> anyhow::Result<()> {
    // build our application with a single route
    let local_path = std::env::current_dir()?;
    let assets_path = local_path.join(Path::new("frontend/assets"));
    let app = Router::new()
        // add a
        .nest_service("/assets", ServeDir::new(assets_path))
        .route("/", get(index_handler));

    // run our app with hyper, listening globally on port 3000
    let port = 3000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .await
        .context("Error while starting server")?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {
        name: "world",
        counter: 0,
    };
    HtmlTemplate(template)
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
