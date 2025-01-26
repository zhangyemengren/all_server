use std::time::Duration;

use crate::routers::{
    bs_routes, cs_routes,
    route::{RouterBuilder, RouteRegister},
    PublicRouter,
};
use axum::{middleware::from_fn, Extension, Router, extract::FromRef};
use tower::ServiceBuilder;
use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::auth::{require_permission, Permission, Role};

///AppState
#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
}

/// 为路由添加权限验证中间件
fn with_auth_middleware(router: Router) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(Extension(Role::guest()))
            .layer(Extension(Permission::ReadBook))
            .layer(from_fn(require_permission))
    )
}

pub async fn new_app() -> Router {
    // 加载.env文件进入环境变量
    dotenvy::dotenv().expect("Failed to load .env file");
    // db相关
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");
    let state = AppState { pool: pool.clone() };
    // 公共路由，不需要权限验证
    let pub_router = RouterBuilder::<PublicRouter>::register();

    // API路由（包含客户端和后台路由）
    let api_router = Router::new()
        .nest("/cs", cs_routes())
        .nest("/bs", bs_routes())
        .merge(pub_router);  // 将公共路由合并到API路由中

    // 合并所有路由，并添加API前缀
    Router::new()
        // .nest("/api", with_auth_middleware(api_router))
        .route("/api", axum::routing::get(|| async { "Hello, World!" }))
        .with_state(state)
}
