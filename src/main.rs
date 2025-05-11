mod models;
mod routes;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let flights = models::load_flights().await.expect("Failed to load flight data");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/flights", get(routes::list_flights))
        .route("/health", get(|| async { "OK" }))
        .layer(cors)
        .with_state(flights);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let host = if std::env::var("PRODUCTION").is_ok() {
        [0, 0, 0, 0] 
    } else {
        [127, 0, 0, 1]
    };
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}