use anyhow::Result;
use axum::{http::StatusCode, routing::get_service, Router};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use web_server::build_website;

#[tokio::main]
async fn main() -> Result<()> {
    // write to a file daily
    let info_file = rolling::daily("logs", "daily.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(info_file);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .json()
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(file_layer)
        .init();

    // the rest of the functionality as a static web builder runs through
    // this function here.
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

    // the address we want for our server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
