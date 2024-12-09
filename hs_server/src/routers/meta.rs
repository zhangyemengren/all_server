use crate::{app::AppState, data::BlizzardLocaleQuery, utils::request_blizzard_api};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension,
};

/// 获取元信息
/// # 例子
/// ```http
/// GET /meta?locale=zh_CN
/// ```
///
/// ```http
/// GET /meta/set?locale=zh_CN
/// ```
pub async fn get_meta(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
    Query(BlizzardLocaleQuery { locale }): Query<BlizzardLocaleQuery>,
    meta: Option<Path<String>>,
) -> impl IntoResponse {
    let client = state.client;
    let meta = meta.map_or("".to_string(), |s| s.0);
    let url = format!("https://us.api.blizzard.com/hearthstone/metadata/{meta}?locale={locale}");
    request_blizzard_api(&client, &url, &token).await
}
