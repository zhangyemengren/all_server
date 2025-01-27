use crate::{
    auth::{require_permission, Permission},
    routers::{health_check, login},
};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower::ServiceBuilder;

///AppState
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn new_app() -> Router {
    // 加载.env文件进入环境变量
    dotenvy::dotenv().expect("Failed to load .env file");
    // db相关
    let db_connection_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");
    let state = AppState { pool: pool.clone() };
    // 公共路由，不需要权限验证
    let pub_router = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login));

    // 客户端路由，需要权限验证
    let cs_router = Router::new()
        .route(
            "/a",
            get(health_check).route_layer(
                ServiceBuilder::new()
                    .layer(Extension(Permission::ReadBook))
                    .layer(from_fn(require_permission)),
            ),
        )
        .route(
            "/b",
            get(health_check).route_layer(
                ServiceBuilder::new()
                    .layer(Extension(Permission::WriteBook))
                    .layer(from_fn(require_permission)),
            ),
        );

    // 后台路由，需要权限验证
    let bs_router = Router::new().route(
        "/c",
        get(health_check).route_layer(
            ServiceBuilder::new()
                .layer(Extension(Permission::ManageUsers))
                .layer(from_fn(require_permission)),
        ),
    );

    // 合并所有路由C端和B端
    let api_router = Router::new()
        .nest("/cs", cs_router)
        .nest("/bs", bs_router)
        .merge(pub_router); // 将公共路由合并到API路由中

    // 合并所有路由，并添加API前缀
    Router::new().nest("/api", api_router).with_state(state)
}
