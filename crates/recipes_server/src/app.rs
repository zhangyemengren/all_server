use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

pub async fn root() -> &'static str {
    "Hello, Axum!"
}

async fn permission_middleware(req: Request, next: Next) -> Result<Response, ()> {
    Ok(next.run(req).await)
}

// cs bs cm
pub fn new_app() -> Router {
    let pub_router: Router<()> = Router::new().nest(
        "/cm",
        Router::new().route("/a", get(root)).route("/b", get(root)),
    );
    let auth_router: Router<()> = Router::new()
        .nest(
            "/cs",
            Router::new().route("/a", get(root)).route("/b", get(root)),
        )
        .nest(
            "/bs",
            Router::new().route("/a", get(root)).route("/b", get(root)),
        )
        .layer(middleware::from_fn(permission_middleware));
    let all = Router::new().merge(pub_router).merge(auth_router);
    all
}
