use server::new_app;

#[tokio::main]
async fn main() {
    let app = new_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
