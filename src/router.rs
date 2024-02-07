use axum::Router;
use tower_http::services::ServeDir;

/// Lists all of the routes contained in the application.
pub fn all() -> Router {
    let serve_static = ServeDir::new("static");

    Router::new().fallback_service(serve_static)
}
