use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, 
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(helthcheck_handle))
        .route("/login", post(login_hanlde));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    password: String,
}

async fn helthcheck_handle() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message":"healthcheck ok"})),
    )
}

async fn login_hanlde(Json(user): Json<User>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"name": user.name, "pass": user.password}))
    )
}