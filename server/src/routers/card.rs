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

fn default_type() -> String {
    "minion".to_string()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_set")]
    pub set: String,
    pub class: String,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_type", rename = "type")]
    pub s_type: String,
    pub mana_cost: String,
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
        s_type,
        mana_cost
    } = params;
    let url = format!(
        "https://us.api.blizzard.com/hearthstone/cards/?\
        set={set}&\
        class={class}&\
        manaCost={mana_cost}&\
        type={s_type}&\
        pageSize={page_size}&\
        page={page}&\
        sort=manaCost:asc,name:asc,classes:asc,groupByClass:asc,groupByClass:asc&\
        locale={locale}",
    );
    request_blizzard_api(&client, &url, &token).await
}
