pub mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new()
        .route("/", get(index::get_self))
        .route("/", post(index::create_user))
        .route("/other", get(index::get_user));

    router
}
