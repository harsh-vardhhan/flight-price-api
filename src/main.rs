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
    // Load flight data from file
    let flights = models::load_flights().await.expect("Failed to load flight data");
    
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/api/flights", get(routes::list_flights))
        .layer(cors)
        .with_state(flights);

    // Run our application
    // Get port from environment variable for cloud deployment (like Render)
    // or use 3000 as default for local development
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    
    // Bind to 0.0.0.0 to listen on all network interfaces (required for cloud deployment)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on http://0.0.0.0:{}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}