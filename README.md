# 🚀 Axum Hello World

A minimal **Axum** web server written in **Rust**, demonstrating how to
structure a small project with routes, controllers, and Tokio runtime.

---

## 📌 Features

- ⚡ **Axum** web framework\
- 🧭 Simple route: `GET /` → `"Hello, World!"`\
- 🗂️ Controller example (`/api/users`)\
- 🔧 Easy to extend and scale\
- 📦 Uses **Tokio** as async runtime

---

## 📁 Project Structure

    .
    ├── src
    │   ├── controllers
    │   │   └── user_controller.rs
    │   │   └── auth_controller.rs
    │   │   └── mod.rs
    │   ├── main.rs
    ├── Cargo.toml
    └── README.md

---

## 🛠️ Installation & Setup

### 1. Install Rust

If you don't have Rust installed:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```sh
rustc --version
```

---

## ▶️ Run the Project

```sh
cargo run
```

Server will start on:

    http://127.0.0.1:3000/

---

## 📄 Example Code

### `src/main.rs`

```rust
use axum::{Router, routing::get};

mod controllers;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", controllers::routes());
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 🧪 Test the Endpoints

### Hello World:

```sh
curl http://localhost:3000/
```

### Users Endpoint:

```sh
curl http://localhost:3000/api/users/
```

---

## 📚 Dependencies

Add these to `Cargo.toml`:

```toml
[dependencies]
axum = "0.8.5"
tokio = { version = "1.48.0", features = ["full"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
```
