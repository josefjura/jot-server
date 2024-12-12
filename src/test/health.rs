use axum_test::TestServer;
use serde_json::json;

use crate::{
    router::setup_router,
    test::{setup_server, JWT_SECRET},
};

#[sqlx::test]
async fn health_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let router = setup_router(db, JWT_SECRET);

    let server = TestServer::new(router).unwrap();

    let response = server.get("/health").await;

    response.assert_status_ok();
}

#[sqlx::test]
async fn health_auth_forbidden(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let response = server.get("/health/auth").await;

    response.assert_status_forbidden();
}

#[sqlx::test(fixtures("user"))]
async fn health_auth_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let login_response = server
        .post("/login")
        .json(&json!({
                "username": "Alice",
                "password": "pass",
        }))
        .await;

    login_response.assert_status_ok();

    let run_response = server.get("/health/auth").await;

    run_response.assert_status_ok();
}
