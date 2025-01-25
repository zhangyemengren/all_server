use crate::auth::{require_permission, Permission, Role};
use crate::routers::route::{Route, RouteRegister};
use axum::{middleware, routing::get, Extension, Router};
use tower::ServiceBuilder;

// 客户端路由枚举
#[derive(Clone, Copy)]
pub enum CSRoute {
    BookList,
    BookDetail,
    UserProfile,
}

impl Route for CSRoute {
    fn path(&self) -> &'static str {
        match self {
            Self::BookList => "/books",
            Self::BookDetail => "/book/detail",
            Self::UserProfile => "/user/profile",
        }
    }

    fn handler(&self) -> axum::routing::MethodRouter {
        match self {
            Self::BookList => get(Self::book_list_handler),
            Self::BookDetail => get(Self::book_detail_handler),
            Self::UserProfile => get(Self::user_profile_handler),
        }
    }

    fn required_permission(&self) -> Option<Permission> {
        Some(match self {
            Self::BookList | Self::BookDetail => Permission::ReadBook,
            Self::UserProfile => Permission::WriteBook,
        })
    }
}

impl CSRoute {
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
}

// 客户端路由注册器
pub struct CSRouter;

impl RouteRegister for CSRouter {
    type RouteType = CSRoute;

    fn routes() -> &'static [Self::RouteType] {
        &[CSRoute::BookList, CSRoute::BookDetail, CSRoute::UserProfile]
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
                        .layer(Extension(Role::guest()))
                        .layer(Extension(permission))
                        .layer(middleware::from_fn(require_permission)),
                ),
            )
        })
    }
}

pub fn cs_routes() -> Router {
    CSRouter::register()
}
