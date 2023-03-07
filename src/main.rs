use axum::{handler::get, Router};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello there testing again"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
