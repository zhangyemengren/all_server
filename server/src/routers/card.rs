use crate::{
    app::AppState,
    utils::{default_locale, request_blizzard_api},
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;

fn default_set() -> String {
    "standard".to_string()
}

fn default_page_size() -> i32 {
    10
}

fn default_page() -> i32 {
    1
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default)]
    pub class: String,
    #[serde(default = "default_set")]
    pub set: String,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    #[serde(default = "default_page")]
    pub page: i32,
}

pub async fn get_cards(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
    Query(params): Query<PageQuery>,
) -> impl IntoResponse {
    let client = state.client;
    let PageQuery {
        locale,
        class,
        set,
        page_size,
        page,
    } = params;
    let url = format!(
        "https://us.api.blizzard.com/hearthstone/cards/?\
        class={class}&\
        set={set}&\
        pageSize={page_size}&\
        page={page}&\
        sort=manaCost:asc,name:asc,classes:asc,groupByClass:asc,groupByClass:asc&\
        locale={locale}",
    );
    request_blizzard_api(&client, &url, &token).await
}
