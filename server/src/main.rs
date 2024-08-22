use server::{get_author_header, get_token};
use std::sync::{LazyLock, Mutex};

pub static TOKEN: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

#[tokio::main]
async fn main() {
    let mut token = TOKEN.lock().unwrap();
    let new_token = get_token().await;
    *token = new_token;
    println!("token {:?}", token);
    let client = reqwest::Client::new();
    let res = client
        .get("https://us.api.blizzard.com/hearthstone/cards/52119-arch-villain-rafaam?locale=zh_CN")
        .headers(get_author_header(&token))
        .send()
        .await
        .unwrap();
    let data = res.json::<serde_json::Value>().await.unwrap();
    println!("data {:?}", data);
}
