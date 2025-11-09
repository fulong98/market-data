use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tracing::info;

pub async fn start_server(port: u16) -> Result<(), std::io::Error> {
    let app = Router::new().route("/health", get(health_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(component = "health_check", "Starting health check server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> Json<Value> {
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};

    let timestamp = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| String::from("unknown"));

    Json(json!({
        "status": "ok",
        "timestamp": timestamp
    }))
}
