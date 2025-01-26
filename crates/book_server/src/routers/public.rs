use axum::routing::{get, post};
use crate::{
    auth::{Permission, Role},
    routers::{
        route::{RouteInfo, RouterConfig},
        login,
    },
};

// 公共路由定义
static ROUTES: &[RouteInfo] = &[
    RouteInfo::new(
        "/login",
        || post(login::login),
        Permission::ReadBook,
    ),
    RouteInfo::new(
        "/health",
        || get(health_check),
        Permission::ReadBook,
    ),
];

async fn health_check() -> &'static str {
    "Service is healthy!"
}

pub struct PublicRouter;

impl RouterConfig for PublicRouter {
    const ROUTES: &'static [RouteInfo] = ROUTES;
    
    fn default_role() -> Role {
        Role::guest()
    }
} 