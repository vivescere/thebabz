use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::controllers;
use crate::state::AppState;

/// Type alias that specializes axum's `Router` to use our state.
type AppRouter = Router<AppState>;

/// Lists all of the routes contained in the application.
pub fn all() -> AppRouter {
    let serve_static = ServeDir::new("static");

    Router::new()
        .merge(landing())
        .fallback_service(serve_static)
}

/// Routes definitions for the `landing` controller.
fn landing() -> AppRouter {
    Router::new()
        .route("/", get(controllers::landing::landing))
}
