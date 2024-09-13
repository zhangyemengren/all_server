use crate::{request_blizzard_api, AppState};
use axum::response::IntoResponse;
use axum::{extract::State, Extension};

pub async fn get_cards(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
) -> impl IntoResponse {
    let client = state.client;
    request_blizzard_api(&client, "https://us.api.blizzard.com/hearthstone/cards/?class=deathknight&set=standard&sort=manaCost:asc,name:asc,classes:asc,groupByClass:asc,groupByClass:asc&locale=zh_CN", &token)
        .await
}
