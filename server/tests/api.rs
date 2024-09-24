use axum::body::to_bytes;
use serde_json::Value;

mod helper;

#[tokio::test]
async fn meta() {
    let res = helper::do_request("/meta", None).await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], 200);
    let card_back_categories = &body["data"]["card_back_categories"].as_array().unwrap();
    assert!(!card_back_categories.is_empty());
}
