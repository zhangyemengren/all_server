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

    /// 获取路由的权限和默认角色
    pub fn get_auth_info(&self, default_role: Role) -> (Permission, Role) {
        (self.permission.clone(), default_role)
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
            router.route(route.path(), route.handler())
        })
    }
}

/// 为路由添加权限验证层
pub trait AuthLayer {
    fn with_auth_layer(self, permission: Permission, role: Role) -> Self;
}

impl AuthLayer for MethodRouter {
    fn with_auth_layer(self, permission: Permission, role: Role) -> Self {
        self.route_layer(
            ServiceBuilder::new()
                .layer(Extension(role))
                .layer(Extension(permission))
                .layer(middleware::from_fn(require_permission))
        )
    }
}
