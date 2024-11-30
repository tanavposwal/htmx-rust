use axum::{routing::get, response::Html, Router, Json};
use askama::Template;
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer};
use std::net::SocketAddr;

// define the template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user).post(create_user))
        .layer(CorsLayer::permissive());

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// Handlers
async fn root() -> Html<String> {
    let template = IndexTemplate {
        title: "Axum with Askama".to_string(),
        message: "Hello, dynamic HTML!".to_string(),
    };
    Html(template.render().unwrap())
}

// GET /user
async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "Tanav".to_string(),
    })
}

// POST /user
async fn create_user(Json(payload): Json<User>) -> Json<User> {
    Json(payload) // Echo back the received user
}
