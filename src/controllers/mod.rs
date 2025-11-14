use axum::Router;

mod auth_controller;
mod user_controller;

pub fn routes() -> Router {
    Router::new()
        .nest("/users", user_controller::routes())
        .nest("/auth", auth_controller::routes())
}
