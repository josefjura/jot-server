use crate::{
    model::repository::Repository,
    test::{self, setup_server},
};

#[sqlx::test(fixtures("user", "repository"))]
async fn repository_get_all_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server.get("/repository").authorization_bearer(token).await;

    response.assert_status_ok();

    let json = response.json::<Vec<Repository>>();

    assert_eq!(2, json.len());
}

#[sqlx::test(fixtures("user"))]
async fn repository_get_by_id_bad_request(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server
        .get("/repository/666")
        .authorization_bearer(token)
        .await;

    response.assert_status_not_found();
}

#[sqlx::test(fixtures("user", "repository"))]
async fn repository_get_by_id_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server
        .get("/repository/1")
        .authorization_bearer(token)
        .await;

    response.assert_status_ok();

    let json = response.json::<Repository>();

    assert_eq!(1, json.id);
}

#[sqlx::test(fixtures("user", "repository"))]
async fn repository_get_by_owner_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server
        .get("/repository/user")
        .authorization_bearer(token)
        .await;

    response.assert_status_ok();

    let json = response.json::<Vec<Repository>>();

    assert_eq!(1, json.len());
}
