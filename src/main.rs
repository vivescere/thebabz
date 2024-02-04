use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
