use axum::Router;
use axum::routing::get;

async fn status() -> &'static str {
    "# Hi"
}

async fn health() -> &'static str {
    "healthy"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/status", get(status))
        .route("/health", get(health));

    let addr = "0.0.0.0:3456";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
