use axum::{Router, response::Json, routing::get};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

// GET /api/users
async fn list_users() -> Json<Value> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".into(),
        },
        User {
            id: 2,
            name: "Bob".into(),
        },
    ];

    Json(json!(users))
}

// GET /api/users
async fn get_user() -> Json<Value> {
    Json(json!(User {
        id: 1,
        name: "Alice".into(),
    }))
}

pub fn routes() -> Router {
    Router::new().route("/", get(list_users))
}
