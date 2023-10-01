use axum::{http::StatusCode, Json};
use serde::Serialize;
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn limited() -> (StatusCode, Json<Message>) {
    let message = Message {
        message: "This is the limited version",
    };
    (StatusCode::OK, Json(message))
}

pub async fn unlimited() -> (StatusCode, Json<Message>) {
    let message = Message {
        message: "You can use this as much as you want",
    };
    (StatusCode::OK, Json(message))
}

#[derive(Serialize)]
pub struct Message {
    message: &'static str,
}
