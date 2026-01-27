use axum::{
    http::Method,
    response::Html,
    routing::{get, post},
    Router,
};
use tower_http::{
    services::ServeDir,
    cors::{CorsLayer, Any}, // import cors modules
};
use std::net::SocketAddr;

mod serial;
use serial::{list_serial_devices, connect, disconnect, send};

#[tokio::main]
async fn main() {
    // Build CORS layer allowing requests
    let cors = CorsLayer::new()
        .allow_origin(Any) // allows any origin (for dev; restrict in prod)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/list", get(list_serial_devices))
        .route("/connect", post(connect))
        .route("/disconnect", post(disconnect))
        .route("/send", post(send))
        .nest_service("/", ServeDir::new("frontend/dist").not_found_service(not_found()))
        .layer(cors); // add the CORS middleware here

    let addr = "0.0.0.0:5000";
    println!("🚀 Server running at http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn not_found() -> axum::routing::MethodRouter {
    get(|| async {
        Html(std::fs::read_to_string("frontend/dist/index.html").unwrap_or_else(|_| {
            "<h1>Not found</h1>".to_string()
        }))
    })
}
