use axum::http::StatusCode;
use serde_json::json;

use crate::{
    model::auth::LoginResponse,
    test::{self, setup_server},
};

use super::login;

#[sqlx::test(fixtures("user"))]
async fn login_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let login_response = server
        .post("/auth/login")
        .json(&json!({
                "username": "alice@email.com",
                "password": "pass",
        }))
        .await;

    login_response.assert_status_ok();

    let _ = login_response.json::<LoginResponse>();
}

#[sqlx::test(fixtures("user"))]
async fn login_unauthorized(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let login_response = server
        .post("/auth/login")
        .json(&json!({
                "username": "alice@email.com",
                "password": "bad_pass",
        }))
        .await;

    login_response.assert_status_unauthorized();
}

#[sqlx::test(fixtures("user"))]
async fn auth_missing_token(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let login_response = server
        .post("/auth/login")
        .json(&json!({
                "username": "alice@email.com",
                "password": "pass",
        }))
        .await;

    login_response.assert_status_ok();

    let json = login_response.json::<LoginResponse>();

    // Call with token
    let run_response = server
        .get("/health/auth")
        .authorization_bearer(json.token.clone())
        .await;

    run_response.assert_status_ok();

    // Call without token
    let run_response = server.get("/health/auth").await;

    run_response.assert_status_forbidden();
}

#[sqlx::test]
async fn auth_device_code(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let response = server
        .post("/auth/device")
        .json(&json!({
            "device_code": "mock_device_code",
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
}

#[sqlx::test(fixtures("device_auth"))]
async fn auth_device_status(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let response = server.get("/auth/status/non-existent-code").await;

    response.assert_status_not_found();
}

#[sqlx::test(fixtures("device_auth"))]
async fn auth_device_status_without_token(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let response = server.get("/auth/status/code-without-token").await;

    response.assert_status(StatusCode::ACCEPTED);
}

#[sqlx::test(fixtures("device_auth"))]
async fn auth_device_status_with_token(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let response = server.get("/auth/status/code-with-token").await;

    response.assert_status_ok();
}

#[sqlx::test(fixtures("user", "device_auth"))]
async fn auth_device_delete_not_found(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = login(&server).await;

    let response = server
        .delete("/auth/device/mock_device_code")
        .authorization_bearer(token)
        .await;

    response.assert_status_not_found();
}

#[sqlx::test(fixtures("user", "device_auth"))]
async fn auth_device_delete(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = login(&server).await;

    let response = server
        .delete("/auth/device/code-with-token")
        .authorization_bearer(token)
        .await;

    response.assert_status(StatusCode::NO_CONTENT);
}
