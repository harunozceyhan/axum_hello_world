use axum::{Router, response::Json, routing::get};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

// GET /api/users
async fn list_auths() -> Json<Value> {
    let users = vec![
        User {
            id: 1,
            name: "Auth1".into(),
        },
        User {
            id: 2,
            name: "Auth2".into(),
        },
    ];

    Json(json!(users))
}

// GET /api/users
async fn get_auth() -> Json<Value> {
    Json(json!(User {
        id: 1,
        name: "Auth1".into(),
    }))
}

pub fn routes() -> Router {
    Router::new().route("/get", get(list_auths))
}
