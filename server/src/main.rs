use server::{get_author_header, get_token,root, new_app};
use std::sync::{LazyLock, Mutex};

pub static TOKEN: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

#[tokio::main]
async fn main() {
    // let mut token = TOKEN.lock().unwrap();
    // let new_token = get_token().await;
    // *token = new_token;
    // println!("token {:?}", token);
    // let client = reqwest::Client::new();
    // let res = client
    //     .get("https://us.api.blizzard.com/hearthstone/cards/?class=deathknight&set=standard&sort=manaCost:asc,name:asc,classes:asc,groupByClass:asc,groupByClass:asc&locale=zh_CN")
    //     .headers(get_author_header(&token))
    //     .send()
    //     .await
    //     .unwrap();
    // let data = res.json::<serde_json::Value>().await.unwrap();
    // println!("data {:?}", data);

    let app = new_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
