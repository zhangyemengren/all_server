use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utils::Validate;

use crate::{app::AppState, response::ResponseData, crypto::Crypto, auth::Role};

#[derive(Validate, Deserialize, Debug)]
pub struct User {
    #[validate(email)]
    email: String,
    #[validate(password)]
    password: String,
}

#[derive(Serialize, Default)]
pub struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct Token {
    id: String,
    email: String,
    role: Role,
    exp: i64,
}

pub async fn login(
    State(_state): State<AppState>,
    Json(payload): Json<User>,
) -> Json<ResponseData<LoginResponse>> {
    let result = payload.validate();
    println!("{:?}", payload);
    match result {
        Ok(_) => {
            println!("User validated");
        }
        Err(e) => {
            println!("User validation failed: {:?}", e);
            return ResponseData::err(1, "User validation failed".to_string());
        }
    }
    let token = Crypto::encode_token(Token {
        id: "".to_string(),
        email: payload.email,
        role: Role::user(),
        exp: 0,
    }).unwrap();
    println!("{:?}", token);
    ResponseData::ok(LoginResponse {
        token,
    })
}
