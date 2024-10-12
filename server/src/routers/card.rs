use crate::{
    app::AppState,
    data::{BlizzardLocaleQuery, Response},
    utils::request_blizzard_api,
};
use axum::extract::Path;
use axum::{
    extract::{Query, State},
    Extension, Json,
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

/// CardsQuery查询参数
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsQuery {
    #[serde(flatten)]
    pub b_wrapper: BlizzardLocaleQuery,
    /// 卡牌集 标准 威兹班的工坊等
    #[serde(default = "default_set")]
    pub set: String,
    /// 每页数量
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    /// 页码
    #[serde(default = "default_page")]
    pub page: i32,
    /// 卡牌类型 随从 法术等
    #[serde(default = "default_type", rename = "type")]
    pub s_type: String,
    /// 法力消耗
    #[serde(default)]
    pub mana_cost: String,
    /// 职业 术士 中立等
    #[serde(default)]
    pub class: String,
    /// 关键字搜索
    #[serde(default)]
    pub text_filter: String,
}
/// 获取卡牌列表
/// # 例子
/// ```http
/// GET /cards?locale=zh_CN&class=warlock&page_size=10&page=1&s_type=minion&manaCost=0&set=standard&textFilter=
/// ```
pub async fn get_cards(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
    Query(params): Query<CardsQuery>,
) -> Result<Json<Response>, axum::http::StatusCode> {
    let client = state.client;
    let CardsQuery {
        b_wrapper: BlizzardLocaleQuery { locale },
        class,
        set,
        page_size,
        page,
        s_type,
        mana_cost,
        text_filter,
    } = params;
    let url = format!(
        "https://us.api.blizzard.com/hearthstone/cards/?\
        textFilter={text_filter}&\
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
/// 获取卡牌详情
/// # 例子
/// ```http
/// GET /cards/80818?locale=zh_CN
/// ```
pub async fn get_card_detail(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
    Query(BlizzardLocaleQuery { locale }): Query<BlizzardLocaleQuery>,
    Path(id): Path<String>,
) -> Result<Json<Response>, axum::http::StatusCode> {
    let client = state.client;
    let url = format!("https://us.api.blizzard.com/hearthstone/cards/{id}?locale={locale}");
    request_blizzard_api(&client, &url, &token).await
}
