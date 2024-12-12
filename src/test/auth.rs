use serde_json::json;

use crate::test::setup_server;

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
async fn logout_ok(db: sqlx::Pool<sqlx::Sqlite>) {
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

    let logout_response = server.post("/logout").await;

    logout_response.assert_status_ok();

    let run_response = server.get("/health/auth").await;

    run_response.assert_status_forbidden();
}
