use axum::Router;
use std::net::SocketAddr;
use tokio;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/", routes::routes());
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // For use with Docker
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
