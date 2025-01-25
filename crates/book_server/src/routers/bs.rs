use crate::auth::{require_permission, Permission, Role};
use crate::routers::route::{Route, RouteRegister};
use axum::{middleware, routing::get, Extension, Router};
use tower::ServiceBuilder;

// 后台路由枚举
#[derive(Clone, Copy)]
pub enum BSRoute {
    Overview,
    Management,
}

impl Route for BSRoute {
    fn path(&self) -> &'static str {
        match self {
            Self::Overview => "/overview",
            Self::Management => "/management",
        }
    }

    fn handler(&self) -> axum::routing::MethodRouter {
        match self {
            Self::Overview => get(Self::overview_handler),
            Self::Management => get(Self::management_handler),
        }
    }

    fn required_permission(&self) -> Option<Permission> {
        Some(match self {
            Self::Overview => Permission::ReadBook,
            Self::Management => Permission::ManageSystem,
        })
    }
}

impl BSRoute {
    // 处理函数实现
    async fn overview_handler() -> &'static str {
        "Book System Overview"
    }

    async fn management_handler() -> &'static str {
        "Book System Management"
    }
}

// 后台路由注册器
pub struct BSRouter;

impl RouteRegister for BSRouter {
    type RouteType = BSRoute;

    fn routes() -> &'static [Self::RouteType] {
        &[BSRoute::Overview, BSRoute::Management]
    }

    // 重写register方法以添加权限中间件
    fn register() -> Router {
        let router = Router::new();
        Self::routes().iter().fold(router, |router, route| {
            let permission = route.required_permission().unwrap_or(Permission::ReadBook);
            router.route(
                route.path(),
                route.handler().route_layer(
                    ServiceBuilder::new()
                        .layer(Extension(Role::admin()))
                        .layer(Extension(permission))
                        .layer(middleware::from_fn(require_permission)),
                ),
            )
        })
    }
}

pub fn bs_routes() -> Router {
    BSRouter::register()
}
