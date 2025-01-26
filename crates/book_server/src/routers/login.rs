use axum::Json;
use serde::Deserialize;
use utils::Validate;

#[derive(Validate, Deserialize, Debug)]
pub struct User {
    #[validate(email)]
    email: String,
    #[validate(password)]
    password: String,
}

pub async fn login(Json(payload): Json<User>) -> &'static str {
    let result = payload.validate();
    println!("{:?}", payload);
    match result {
        Ok(_) => {
            println!("User validated");
            "User validated"
        }
        Err(e) => {
            println!("User validation failed: {:?}", e);
            "User validation failed"
        }
    }
}
