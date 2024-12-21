use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Extension, Router,
};
use http::StatusCode;
use tower::ServiceBuilder;

pub async fn root() -> &'static str {
    "Hello, Axum!"
}

// TODO 最小实现
#[derive(Clone, Debug)]
pub enum Permission {
    Admin,
    User,
    Guest,
}
// 先校验登录态 在校验用户权限
async fn permission_middleware(
    Extension(permission): Extension<Permission>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("permission {:?}", permission);
    // match permission {
    //     Permission::Admin => Ok(()),
    //     Permission::User => Err(StatusCode::FORBIDDEN),
    // }?;
    Ok(next.run(req).await)
}

// cs bs cm
pub fn new_app() -> Router {
    let pub_router: Router<()> = Router::new().nest(
        "/cm",
        Router::new().route("/a", get(root)).route("/b", get(root)),
    );
    let cs_routers: Router<()> = Router::new()
        .route("/a", get(root))
        .route_layer(
            ServiceBuilder::new()
                .layer(Extension(Permission::User))
                .layer(middleware::from_fn(permission_middleware)),
        )
        .route("/b", get(root));
    let bs_routers: Router<()> = Router::new()
        .route("/a", get(root))
        .route_layer(
            ServiceBuilder::new()
                .layer(Extension(Permission::Admin))
                .layer(middleware::from_fn(permission_middleware)),
        )
        .route("/b", get(root));
    let nest_routers: Router<()> = Router::new()
        .nest("/cs", cs_routers)
        .nest("/bs", bs_routers);
    let all = Router::new().merge(pub_router).merge(nest_routers);
    all
}
