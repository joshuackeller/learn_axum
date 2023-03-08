pub mod auth;
pub mod base;
pub mod users;

use axum::Router;

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new()
        .nest("/", base::routes())
        .nest("/users", users::routes())
        .nest("/auth", auth::routes());

    router
}
