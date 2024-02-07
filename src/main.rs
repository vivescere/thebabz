use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

mod state;
mod errors;
mod routes;
mod controllers;

#[tokio::main]
async fn main() {
    let _guard = setup_tracing();

    tracing::info!("Starting application");

    let app_state: state::AppState = state::AppState::new();

    let app = routes::all()
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("should be able to listen on the specified port");

    axum::serve(listener, app)
        .await
        .expect("should be able to serve");
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
