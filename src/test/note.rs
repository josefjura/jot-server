use crate::{
    model::note::Note,
    test::{self, setup_server},
};

#[sqlx::test(fixtures("user", "repository", "note"))]
async fn note_get_all_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server.get("/note").authorization_bearer(token).await;

    response.assert_status_ok();

    let json = response.json::<Vec<Note>>();

    assert_eq!(4, json.len());
}

#[sqlx::test(fixtures("user"))]
async fn note_get_by_id_bad_request(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server.get("/note/666").authorization_bearer(token).await;

    response.assert_status_not_found();
}

#[sqlx::test(fixtures("user", "repository", "note"))]
async fn note_get_by_id_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server.get("/note/1").authorization_bearer(token).await;

    response.assert_status_ok();

    let json = response.json::<Note>();

    assert_eq!(1, json.id);
}

#[sqlx::test(fixtures("user", "repository", "note"))]
async fn note_get_by_owner_ok(db: sqlx::Pool<sqlx::Sqlite>) {
    let server = setup_server(db);

    let token = test::login(&server).await;

    let response = server.get("/user/note").authorization_bearer(token).await;

    response.assert_status_ok();

    let json = response.json::<Vec<Note>>();

    assert_eq!(2, json.len());
}
