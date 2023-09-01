use axum::{http::StatusCode, routing::get_service, Router};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Result;

use web_server::build_website;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

	build_website("content", "public")?;

    let app = Router::new()
        .nest(
            "/",
			// we are serving the public directory here.
            get_service(ServeDir::new("public")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .layer(TraceLayer::new_for_http());

    // 0.0.0.0 so we can use the base for now...
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("Listening on {}", addr);
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

	Ok(())
}
