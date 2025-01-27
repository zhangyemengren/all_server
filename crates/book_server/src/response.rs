use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseData<T: Serialize> {
    pub code: u8,
    pub data: T,
    pub message: String,
}

impl<T: Serialize> ResponseData<T> {
    pub fn new(code: u8, message: String, data: T) -> Json<Self> {
        Json(Self {
            code,
            message,
            data,
        })
    }

    pub fn ok(data: T) -> Json<Self> {
        Json(Self {
            code: 0,
            message: "success".to_string(),
            data,
        })
    }

    pub fn err(code: u8, message: String) -> Json<Self>
    where
        T: Default,
    {
        Json(Self {
            code,
            message,
            data: T::default(),
        })
    }
}
