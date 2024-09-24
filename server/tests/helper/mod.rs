use axum::http::header;
use axum::{
    body::Body,
    http::{Method, Request},
    response::Response,
};
use server::app::new_app;
use server::utils::{get_author_header, new_token};
use tower::ServiceExt;

pub async fn do_request(uri: &str, body: Option<Body>) -> Response {
    let body = body.unwrap_or(Body::empty());
    let app = new_app().await;
    let token = new_token().await;
    let headers = get_author_header(&token);
    app.oneshot(
        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(
                header::AUTHORIZATION,
                headers
                    .get(header::AUTHORIZATION)
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .body(body)
            .unwrap(),
    )
    .await
    .unwrap()
}
