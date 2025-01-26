use crate::routers::{
    bs_routes, cs_routes,
    route::{RouterBuilder, RouteRegister},
    PublicRouter,
};
use axum::{middleware::from_fn, Extension, Router};
use tower::ServiceBuilder;
use crate::auth::{require_permission, Permission, Role};

/// 为路由添加权限验证中间件
fn with_auth_middleware(router: Router) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(Extension(Role::guest()))
            .layer(Extension(Permission::ReadBook))
            .layer(from_fn(require_permission))
    )
}

pub fn new_app() -> Router {
    // 公共路由，不需要权限验证
    let pub_router = RouterBuilder::<PublicRouter>::register();

    // API路由（包含客户端和后台路由）
    let api_router = Router::new()
        .nest("/cs", cs_routes())
        .nest("/bs", bs_routes())
        .merge(pub_router);  // 将公共路由合并到API路由中

    // 合并所有路由，并添加API前缀
    Router::new()
        .nest("/api", with_auth_middleware(api_router))
}
