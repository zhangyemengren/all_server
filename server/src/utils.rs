use crate::data::Response;
use axum::Json;

pub fn get_env_var(key: &str) -> String {
    dotenvy::dotenv().ok();
    if let Ok(v) = std::env::var(key) {
        return v;
    }

    "".to_string()
}

pub async fn get_token(state: crate::AppState) -> String {
    let token = state.token.lock().await;
    token.clone()
}
pub async fn set_token(state: crate::AppState) -> String {
    let new_token = new_token().await;
    let mut token = state.token.lock().await;
    *token = new_token.clone();
    new_token
}
pub async fn new_token() -> String {
    let client_id = get_env_var("client_id");
    let client_secret = get_env_var("client_secret");

    let mut params = std::collections::HashMap::new();
    params.insert("grant_type", "client_credentials");
    let client = reqwest::Client::new();
    let res = client
        .post("https://oauth.battle.net/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await
        .unwrap();
    let data = res.json::<serde_json::Value>().await.unwrap();
    data["access_token"].as_str().unwrap().to_string()
}

pub fn get_author_header(token: &str) -> axum::http::HeaderMap {
    use axum::http::HeaderMap;

    let author = format!("Bearer {}", token);
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", author.parse().unwrap());
    headers
}

pub async fn request_blizzard_api(
    client: &reqwest::Client,
    url: &str,
    token: &str,
) -> Result<Json<Response>, axum::http::StatusCode> {
    use axum::http::StatusCode;
    use serde_json::Value;

    let res = client
        .get(url)
        .headers(get_author_header(token))
        .send()
        .await;
    let Ok(res) = res else {
        println!("request_blizzard_api error: {:?}", res);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    match res.status() {
        StatusCode::UNAUTHORIZED => Err(StatusCode::UNAUTHORIZED),
        StatusCode::OK => {
            if let Ok(json) = res.json::<Value>().await {
                Response::ok(json).into_axum_response()
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        any_code => Err(any_code),
    }
}
