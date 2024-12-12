use axum_test::TestServer;
use serde_json::json;

use crate::router::setup_router;

mod auth;
mod health;
mod repository;

const JWT_SECRET: &str = "BrHTysKWKIhwOTyqYvqEUOf3rhTH06Q3k2ZBf3Zbcew=";

pub async fn login(server: &TestServer) {
    let login_response = server
        .post("/login")
        .json(&json!({
                        "username": "Alice",
                        "password": "pass",
        }))
        .await;

    login_response.assert_status_ok();
}

pub fn setup_server(db: sqlx::Pool<sqlx::Sqlite>) -> TestServer {
    let router = setup_router(db, JWT_SECRET);

    TestServer::builder().save_cookies().build(router).unwrap()
}
