use axum::routing::get;
use axum::Router;

pub async fn root() -> &'static str {
    "Hello, Axum!"
}

// cs bs cm
pub fn new_app() -> Router {
    Router::new().route("/", get(root))
}
