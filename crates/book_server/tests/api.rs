mod helper;
use axum::body::to_bytes;
use serde_json::Value;

#[tokio::test]
async fn test_login() {
    let res = helper::do_login("/api/login", "test@test.com", "123456qwE!").await;
    let email_err_res = helper::do_login("/api/login", "test.com", "123456qwE!").await;
    assert_eq!(res.status(), 200);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    println!("{:?}", body);

    assert_eq!(email_err_res.status(), 200); // 邮箱错误 暂时200 后续区分业务或http状态码
}
