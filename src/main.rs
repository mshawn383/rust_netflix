use axum::{
    routing::get,
    Router,
};
mod api;
use crate::api::live_streaming::http_live_streaming;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
    .layer(CorsLayer::new().allow_origin(Any))
    .route("/media/{filename}", get(http_live_streaming));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}