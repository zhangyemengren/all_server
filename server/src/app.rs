use axum::Router;
use axum::routing::get;
use crate::root;

pub async fn new_app() -> Router{
    Router::new()
        .route("/", get(root))
}