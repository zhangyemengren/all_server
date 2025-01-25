use crate::auth::Permission;
use axum::{routing::MethodRouter, Router};

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
