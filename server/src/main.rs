use std::sync::{LazyLock, Mutex};

static TOKEN: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

fn main(){
    let mut token = TOKEN.lock().unwrap();
    println!("Hello, world! {}", token);
    *token = "1234".to_string();
    println!("token will be {}", token);
}