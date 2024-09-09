use crate::{get_cards, root};
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub(crate) token: Arc<Mutex<String>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            token: Arc::new(Mutex::new("".to_string())),
        }
    }
}
pub async fn new_app() -> Router {
    let state = AppState::new();
    Router::new()
        .route("/", get(root))
        .route("/cards", get(get_cards))
        .with_state(state)
}
