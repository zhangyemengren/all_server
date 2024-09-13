use crate::{
    app::AppState,
    utils::{default_locale, request_blizzard_api},
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_locale")]
    pub locale: String,
}

pub async fn get_meta(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
    Query(params): Query<PageQuery>,
    meta: Option<Path<String>>,
) -> impl IntoResponse {
    let client = state.client;
    let meta = meta.map_or("".to_string(), |s| s.0);
    let url = format!(
        "https://us.api.blizzard.com/hearthstone/metadata/{}?locale={}",
        meta, params.locale
    );
    request_blizzard_api(&client, &url, &token).await
}
