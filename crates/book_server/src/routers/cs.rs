use crate::auth::{Permission, Role};
use crate::routers::route::{RouteInfo, RouterBuilder, RouterConfig, RouteRegister};
use axum::routing::get;

// 路由定义
static ROUTES: &[RouteInfo] = &[
    RouteInfo::new(
        "/books",
        || get(book_list_handler),
        Permission::ReadBook,
    ),
    RouteInfo::new(
        "/book/detail",
        || get(book_detail_handler),
        Permission::ReadBook,
    ),
    RouteInfo::new(
        "/user/profile",
        || get(user_profile_handler),
        Permission::WriteBook,
    ),
];

// 处理函数实现
async fn book_list_handler() -> &'static str {
    "Book List"
}

async fn book_detail_handler() -> &'static str {
    "Book Detail"
}

async fn user_profile_handler() -> &'static str {
    "User Profile"
}

pub struct CSRouter;

impl RouterConfig for CSRouter {
    const ROUTES: &'static [RouteInfo] = ROUTES;
    
    fn default_role() -> Role {
        Role::guest()
    }
}

pub fn cs_routes() -> axum::Router {
    RouterBuilder::<CSRouter>::register()
}
