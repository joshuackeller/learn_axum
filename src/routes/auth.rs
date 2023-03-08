pub mod index;

use axum::{routing::post, Router};

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new().route("/sign_in", post(index::sign_in));

    router
}
