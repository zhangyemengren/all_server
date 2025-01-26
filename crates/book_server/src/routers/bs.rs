use crate::auth::{Permission, Role};
use crate::routers::route::{RouteInfo, RouterBuilder, RouterConfig, RouteRegister};
use axum::routing::get;

// 路由定义
static ROUTES: &[RouteInfo] = &[
    RouteInfo::new(
        "/overview",
        || get(overview_handler),
        Permission::ReadBook,
    ),
    RouteInfo::new(
        "/management",
        || get(management_handler),
        Permission::ManageSystem,
    ),
];

// 处理函数实现
async fn overview_handler() -> &'static str {
    "Book System Overview"
}

async fn management_handler() -> &'static str {
    "Book System Management"
}

pub struct BSRouter;

impl RouterConfig for BSRouter {
    const ROUTES: &'static [RouteInfo] = ROUTES;
    
    fn default_role() -> Role {
        Role::admin()
    }
}

pub fn bs_routes() -> axum::Router {
    RouterBuilder::<BSRouter>::register()
}
