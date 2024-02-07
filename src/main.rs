use tower_http::trace::TraceLayer;
use tracing::{event, Level};
use tracing_appender::non_blocking::WorkerGuard;

mod state;
mod routes;
mod controllers;

#[tokio::main]
async fn main() {
    let _guard = setup_tracing();

    event!(Level::INFO, "Starting application");

    let app_state: state::AppState = state::AppState::new();

    let app = routes::all()
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn setup_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "thebabz.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::DEBUG)
        .init();

    guard
}
