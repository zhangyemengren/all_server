use crate::{
    routers::{card::get_cards, meta::get_meta, root::root},
    utils::{get_token, set_token},
};
use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub(crate) token: Arc<Mutex<String>>,
    pub(crate) client: reqwest::Client,
}

impl AppState {
    fn new() -> Self {
        AppState {
            token: Arc::new(Mutex::new("".to_string())),
            client: reqwest::Client::new(),
        }
    }
}

async fn refresh_token(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let (parts, body) = request.into_parts();
    // body没有实现Clone，所以需要转换为bytes 通过bytes创建新的Request<Body>
    let body_bytes = to_bytes(body, usize::MAX).await.unwrap();
    let mut req_clone = Request::from_parts(parts.clone(), Body::from(body_bytes.clone()));
    let token = get_token(state.clone()).await;
    // 执行handler
    let mut request = Request::from_parts(parts, Body::from(body_bytes));
    request.extensions_mut().insert(token);
    let response = next.clone().run(request).await;
    // 4xx 刷新token 重发一次请求
    if response.status().is_client_error() {
        println!("refresh_token");
        // 封装了 单独使用需要注意Mutex释放
        let token = set_token(state).await;
        req_clone.extensions_mut().insert(token);
        let retry_response = next.run(req_clone).await;
        return retry_response;
    }
    response
}
pub async fn new_app() -> Router {
    let state = AppState::new();
    Router::new()
        .route("/", get(root))
        .route("/cards", get(get_cards))
        .route("/meta", get(get_meta))
        .route("/meta/:type", get(get_meta))
        .route_layer(middleware::from_fn_with_state(state.clone(), refresh_token))
        .with_state(state)
}
