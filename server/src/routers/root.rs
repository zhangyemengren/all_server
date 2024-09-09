use crate::{get_author_header, get_token, AppState};
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

pub async fn root() -> &'static str {
    "ok"
}

pub async fn get_cards(State(state): State<AppState>) -> Json<Value> {
    let new_token = get_token().await;
    let mut token = state.token.lock().await;
    *token = new_token;
    println!("token {:?}", token);
    let client = reqwest::Client::new();
    let res = client
        .get("https://us.api.blizzard.com/hearthstone/cards/?class=deathknight&set=standard&sort=manaCost:asc,name:asc,classes:asc,groupByClass:asc,groupByClass:asc&locale=zh_CN")
        .headers(get_author_header(&token))
        .send()
        .await
        .unwrap();
    let data = res.json::<Value>().await.unwrap();
    println!("data {:?}", data);
    data.into()
}
