pub mod index;

use axum::{routing::get, Router};

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new().route("/", get(index::health_check));

    router
}
