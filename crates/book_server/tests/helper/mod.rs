use axum::http::{header, HeaderMap};
use axum::{
    body::Body,
    http::{Method, Request},
    response::Response,
};
use book_server::app::new_app;
use tower::ServiceExt;

pub fn get_author_header(token: &str) -> axum::http::HeaderMap {
    let author = format!("Bearer {}", token);
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", author.parse().unwrap());
    headers
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

// pub async fn do_request(uri: &str, body: Option<Body>) -> Response {
//     let body = body.unwrap_or(Body::empty());
//     let app = new_app().await;
//     let token = new_token().await;
//     let headers = get_author_header(&token);
//     app.oneshot(
//         Request::builder()
//             .method(Method::GET)
//             .uri(uri)
//             .header(
//                 header::AUTHORIZATION,
//                 headers
//                     .get(header::AUTHORIZATION)
//                     .unwrap()
//                     .to_str()
//                     .unwrap(),
//             )
//             .body(body)
//             .unwrap(),
//     )
//     .await
//     .unwrap()
// }
