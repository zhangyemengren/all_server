use crate::auth::{require_permission, Permission, Role};
use axum::{middleware, routing::MethodRouter, Extension, Router};
use tower::ServiceBuilder;

/// 路由特征，定义路由的基本行为
pub trait Route: Clone + 'static {
    /// 获取路由路径
    fn path(&self) -> &'static str;
    /// 获取路由处理函数
    fn handler(&self) -> MethodRouter;
    /// 获取路由所需权限，默认为None表示不需要权限
    fn required_permission(&self) -> Option<Permission> {
        None
    }
}

#[derive(Clone)]
pub struct RouteInfo {
    path: &'static str,
    handler: fn() -> MethodRouter,
    permission: Permission,
}

impl RouteInfo {
    pub const fn new(path: &'static str, handler: fn() -> MethodRouter, permission: Permission) -> Self {
        Self {
            path,
            handler,
            permission,
        }
    }
}

impl Route for RouteInfo {
    fn path(&self) -> &'static str {
        self.path
    }

    fn handler(&self) -> MethodRouter {
        (self.handler)()
    }

    fn required_permission(&self) -> Option<Permission> {
        Some(self.permission.clone())
    }
}

pub trait RouterConfig {
    const ROUTES: &'static [RouteInfo];
    fn default_role() -> Role;
}

// 使用PhantomData帮助处理为使用的类型参数
pub struct RouterBuilder<T: RouterConfig> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: RouterConfig> Default for RouterBuilder<T> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

/// 路由集合特征，用于批量注册路由
pub trait RouteRegister {
    type RouteType: Route;

    /// 获取所有路由
    fn routes() -> &'static [Self::RouteType];

    /// 注册所有路由
    fn register() -> Router {
        let router = Router::new();
        Self::routes().iter().fold(router, |router, route| {
            router.route(route.path(), route.handler())
        })
    }
}

impl<T: RouterConfig> RouteRegister for RouterBuilder<T> {
    type RouteType = RouteInfo;

    fn routes() -> &'static [Self::RouteType] {
        T::ROUTES
    }

    fn register() -> Router {
        let router = Router::new();
        Self::routes().iter().fold(router, |router, route| {
            let permission = route.required_permission().unwrap_or(Permission::ReadBook);
            router.route(
                route.path(),
                route.handler().route_layer(
                    ServiceBuilder::new()
                        .layer(Extension(T::default_role()))
                        .layer(Extension(permission))
                        .layer(middleware::from_fn(require_permission)),
                ),
            )
        })
    }
}
