use axum::body::to_bytes;
use serde_json::Value;
use whatlang::{detect, Lang};

mod helper;

#[tokio::test]
async fn meta_all() {
    let res = helper::do_request("/meta", None).await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], 200);
    let card_back_categories = &body["data"]["cardBackCategories"].as_array().unwrap();
    assert!(!card_back_categories.is_empty());
}

#[tokio::test]
async fn meta_sets() {
    let res = helper::do_request("/meta/sets", None).await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], 200);
    let sets = &body["data"].as_array().unwrap();
    assert!(!sets.is_empty());
}

#[tokio::test]
async fn meta_types_jp() {
    let res = helper::do_request("/meta/types?locale=ja_JP", None).await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], 200);
    let types = &body["data"].as_array().unwrap();
    assert!(!types.is_empty());
    let name = &types[0]["name"];
    let info = detect(name.as_str().unwrap()).unwrap();
    assert_eq!(info.lang(), Lang::Jpn);
}

#[tokio::test]
async fn cards() {
    let res = helper::do_request("/cards?s_type=minion&manaCost=5", None).await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], 200);
    let cards = &body["data"]["cards"].as_array().unwrap();
    assert!(!cards.is_empty());
}

#[tokio::test]
async fn cards_detail() {
    let res = helper::do_request("/cards/80818?locale=de_DE", None).await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], 200);
    let name = &body["data"]["name"].as_str().unwrap();
    assert!(!name.is_empty());
    let info = detect(name).unwrap();
    assert_eq!(info.lang(), Lang::Deu);
}
