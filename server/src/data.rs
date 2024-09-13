use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Response {
    pub status: i32,
    pub data: Value,
    pub message: String,
}

impl Response {
    pub fn new(status: i32, data: Value, message: String) -> Self {
        Response {
            status,
            data,
            message,
        }
    }
    pub fn ok(data: Value) -> Self {
        Response {
            status: 200,
            data,
            message: "Success".to_string(),
        }
    }
    pub fn into_axum_response(self) -> Result<Json<Response>, StatusCode> {
        Ok(Json(self))
    }
}
