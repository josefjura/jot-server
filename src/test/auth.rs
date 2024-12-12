use serde_json::json;

use crate::{model::auth::LoginResponse, test::setup_server};

#[sqlx::test(fixtures("user"))]
async fn login_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let login_response = server
        .post("/login")
        .json(&json!({
                "username": "Alice",
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
        .post("/login")
        .json(&json!({
                "username": "Alice",
                "password": "bad_pass",
        }))
        .await;

    login_response.assert_status_unauthorized();
}

#[sqlx::test(fixtures("user"))]
async fn auth_missing_token(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let login_response = server
        .post("/login")
        .json(&json!({
                "username": "Alice",
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
