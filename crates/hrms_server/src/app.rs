use crate::{
    auth::{require_permission, Permission},
    routers::{health_check, login},
};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
use http::HeaderValue;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

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
    // 配置CORS中间件
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("https://hrms-henna.vercel.app"))
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(false);

    // 公共路由，不需要权限验证
    let pub_router = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login));


    // 后台路由，需要权限验证
    let auth_router = Router::new().route(
        "/a",
        get(health_check).route_layer(
            ServiceBuilder::new()
                .layer(Extension(Permission::ManageUsers))
                .layer(from_fn(require_permission)),
        ),
    );

    // 合并所有API路由
    let api_routes = Router::new()
        .merge(pub_router)
        .merge(auth_router);

    // 应用前缀、状态和CORS
    Router::new()
        .nest("/api", api_routes)  // 所有路由都带/api前缀
        .with_state(state)
        .layer(cors)
}
