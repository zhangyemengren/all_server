use axum::body::to_bytes;
use axum::http::{header, HeaderMap};
use axum::{
    body::Body,
    http::{Method, Request},
    response::Response,
};
use book_server::app::new_app;
use serde_json::Value;
use tower::ServiceExt;

pub async fn do_login_and_get_token(email: &str, password: &str) -> String {
    let res = do_login("/api/login", email, password).await;
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    body["data"]["token"].as_str().unwrap().to_string()
}

pub async fn do_login(uri: &str, email: &str, password: &str) -> Response {
    let app = new_app().await;
    let body = Body::from(
        serde_json::json!({
            "email": email,
            "password": password
        })
        .to_string(),
    );

    app.oneshot(
        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn do_request(uri: &str, token: &str, body: Option<Body>) -> Response {
    let body = body.unwrap_or(Body::empty());
    let app = new_app().await;
    app.oneshot(
        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", token),
            )
            .body(body)
            .unwrap(),
    )
    .await
    .unwrap()
}
