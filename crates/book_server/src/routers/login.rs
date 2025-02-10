use std::time::Duration;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utils::Validate;
use jiff::Timestamp;

use crate::{app::AppState, response::ResponseData, crypto::Crypto, auth::Role};

#[derive(Validate, Deserialize, Debug)]
pub struct User {
    #[validate(email)]
    username: String,
    #[validate(password)]
    password: String,
}

#[derive(Serialize, Default)]
pub struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct Token {
    email: String,
    role: Role,
    exp: i64,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            email: "".to_string(),
            role: Role::guest(),
            exp: (Timestamp::now() + Duration::from_secs(3600)).as_second(),
        }
    }
}

#[derive(Debug)]
pub struct DBUser {
    username: String,
    permission_id: i32,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Json<ResponseData<LoginResponse>> {
    let result = payload.validate();
    println!("{:?}", payload);
    if let Err(e) = result {
        println!("User validation failed: {:?}", e);
        return ResponseData::err(1, "User validation failed".to_string());
    }

    let user = sqlx::query_as!(
        DBUser,
        "SELECT * FROM users WHERE username = $1 AND password = $2 LIMIT 1",
        payload.username,
        payload.password
    )
    .fetch_optional(&state.pool)
    .await;

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            return ResponseData::err(1, "username or password is incorrect".to_string());
        }
        Err(e) => {
            println!("Database error: {:?}", e);
            return ResponseData::err(1, "database error".to_string());
        }
    };
    println!("{:?}", user);

    let token = Crypto::encode_token(Token {
        email: user.username,
        role: Role::from_db_user(user.permission_id),
        ..Default::default()
    }).unwrap();

    ResponseData::ok(LoginResponse {
        token,
    })
}
