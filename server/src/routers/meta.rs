use crate::{request_blizzard_api, AppState};
use axum::response::IntoResponse;
use axum::{extract::State, Extension};

pub async fn get_meta_sets(
    State(state): State<AppState>,
    Extension(token): Extension<String>,
) -> impl IntoResponse {
    let client = state.client;
    request_blizzard_api(
        &client,
        "https://us.api.blizzard.com/hearthstone/metadata/sets?locale=zh_CN",
        &token,
    )
    .await
}
