use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utils::Validate;

use crate::{app::AppState, response::ResponseData};

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

pub async fn login(
    State(state): State<AppState>,
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
    ResponseData::ok(LoginResponse {
        token: "".to_string(),
    })
}
