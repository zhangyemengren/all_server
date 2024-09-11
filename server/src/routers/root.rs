use crate::{get_author_header};
use axum::{http::StatusCode, response::Json, Extension};
use serde_json::Value;

pub async fn root() -> &'static str {
    "ok"
}

pub async fn get_cards(Extension(token): Extension<String>) -> Result<Json<Value>, StatusCode> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://us.api.blizzard.com/hearthstone/cards/?class=deathknight&set=standard&sort=manaCost:asc,name:asc,classes:asc,groupByClass:asc,groupByClass:asc&locale=zh_CN")
        .headers(get_author_header(&token))
        .send()
        .await
        .unwrap();
    if res.status().is_client_error() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let data = res.json::<Value>().await.unwrap();
    println!("data {:?}", data);
    Ok(Json(data))
}

pub async fn get_meta_sets(Extension(token): Extension<String>) -> Result<Json<Value>, StatusCode> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://us.api.blizzard.com/hearthstone/metadata/sets?locale=zh_CN")
        .headers(get_author_header(&token))
        .send()
        .await
        .unwrap();
    if res.status().is_client_error() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let data = res.json::<Value>().await.unwrap();
    println!("data {:?}", data);
    Ok(Json(data))
}