use crate::routers::{
    bs_routes, cs_routes,
    route::{Route, RouteRegister},
    validate_user,
};
use axum::{routing::get, Router};

// 公共路由枚举
#[derive(Clone, Copy)]
pub enum CMRoute {
    Login,
    Health,
}

impl Route for CMRoute {
    fn path(&self) -> &'static str {
        match self {
            Self::Login => "/login",
            Self::Health => "/health",
        }
    }

    fn handler(&self) -> axum::routing::MethodRouter {
        match self {
            Self::Login => get(validate_user),
            Self::Health => get(health_check),
        }
    }
}

// 公共路由注册器
pub struct CMRouter;

impl RouteRegister for CMRouter {
    type RouteType = CMRoute;

    fn routes() -> &'static [Self::RouteType] {
        &[CMRoute::Login, CMRoute::Health]
    }
}

async fn health_check() -> &'static str {
    "Service is healthy!"
}

pub fn new_app() -> Router {
    // 公共路由，不需要权限验证
    let pub_router = Router::new().nest("/cm", CMRouter::register());

    // API路由（包含客户端和后台路由）
    let api_router = Router::new()
        .nest("/cs", cs_routes())
        .nest("/bs", bs_routes());

    // 合并所有路由，并添加API前缀
    Router::new().merge(pub_router).nest("/api", api_router)
}
