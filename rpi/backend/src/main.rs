use axum::{
    response::Html,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

mod usb;
use usb::list_usb_devices;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/usb", get(list_usb_devices))
        .nest_service("/", ServeDir::new("frontend/dist").not_found_service(not_found()));

    let addr = "0.0.0.0:5000";
    println!("🚀 Server running at http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Return index.html for unknown routes (SPA support)
fn not_found() -> axum::routing::MethodRouter {
    get(|| async {
        Html(std::fs::read_to_string("frontend/dist/index.html").unwrap_or_else(|_| {
            "<h1>Not found</h1>".to_string()
        }))
    })
}
