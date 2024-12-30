use aide::{
    axum::{
        routing::{delete_with, get_with, post_with},
        ApiRouter, IntoApiResponse,
    },
    transform::TransformOperation,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db,
    errors::RestError,
    model::{
        note::{CreateNoteRequest, DeleteManyRequest, Note, NoteSearchRequest},
        user::User,
    },
    state::AppState,
};

use super::with_auth_middleware;

pub fn note_routes(app_state: AppState) -> ApiRouter<AppState> {
    let router = ApiRouter::new()
        .api_route(
            "/note",
            get_with(get_all, get_all_docs).post_with(create, create_docs),
        )
        .api_route("/note/delete", delete_with(delete_many, delete_many_docs))
        .api_route("/note/search", post_with(post_search, post_search_docs))
        .api_route(
            "/note/:id",
            get_with(get_by_id, get_by_id_docs).delete_with(delete, delete_docs),
        )
        .api_route(
            "/user/note",
            get_with(get_all_by_owner, get_all_by_owner_docs),
        );

    with_auth_middleware(router, app_state)
}

pub async fn get_all(State(state): State<AppState>) -> impl IntoApiResponse {
    let items = db::notes::get_all(state.db).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    match items {
        Ok(items) => Json(items).into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn get_all_docs(op: TransformOperation) -> TransformOperation {
    op.description("Retrieve all existing notes")
        .tag("Note")
        .response_with::<200, Json<Vec<Note>>, _>(|res| {
            res.example(vec![
                Note {
                    id: 1,
                    content: "Some note".to_string(),
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                    user_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    target_date: Utc::now().date_naive(),
                },
                Note {
                    id: 1,
                    content: "Some other note".to_string(),
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                    user_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    target_date: Utc::now().date_naive(),
                },
            ])
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn get_all_by_owner(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoApiResponse {
    let items = db::notes::get_all_by_user(state.db, user.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get all repositories: {:?}", e);
            RestError::Database(e)
        });

    match items {
        Ok(items) => Json(items).into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn get_all_by_owner_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Retrieve user notes")
        .description("Retrieve all notes owned by the authenticated user")
        .tag("User")
        .response_with::<200, Json<Vec<Note>>, _>(|res| {
            res.example(vec![
                Note {
                    id: 1,
                    content: "Some note".to_string(),
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                    user_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    target_date: Utc::now().date_naive(),
                },
                Note {
                    id: 1,
                    content: "Some other note".to_string(),
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                    user_id: 2,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    target_date: Utc::now().date_naive(),
                },
            ])
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
) -> impl IntoApiResponse {
    let result = db::notes::delete(state.db, id, user.id).await.map_err(|e| {
        tracing::error!("Failed to delete notes: {:?}", e);
        RestError::Database(e)
    });

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn delete_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Delete note")
        .description("Delete a note by its ID")
        .tag("Note")
        .response_with::<204, (), _>(|res| res.description("Note deleted successfully"))
        .response_with::<500, (), _>(|res| {
            res.description("Database error occurred while deleting note")
        })
}

pub async fn delete_many(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<DeleteManyRequest>,
) -> impl IntoApiResponse {
    let result = db::notes::delete_many(state.db, &request.ids, user.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete notes: {:?}", e);
            RestError::Database(e)
        });

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn delete_many_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Delete notes")
        .description("Delete multiple notes by their IDs")
        .tag("Note")
        .response_with::<204, (), _>(|res| res.description("Notes deleted successfully"))
        .response_with::<500, (), _>(|res| {
            res.description("Database error occurred while deleting notes")
        })
}

pub async fn get_by_id(Path(id): Path<i64>, State(state): State<AppState>) -> impl IntoApiResponse {
    let item = db::notes::get_by_id(state.db, id).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    match item {
        Ok(Some(item)) => Json(item).into_response(),
        Ok(None) => RestError::NotFound.into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn get_by_id_docs(op: TransformOperation) -> TransformOperation {
    op.description("Retrieve a note by its ID")
        .tag("Note")
        .response_with::<200, Json<Note>, _>(|res| {
            res.example(Note {
                id: 1,
                content: "Some note".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                user_id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                target_date: Utc::now().date_naive(),
            })
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<CreateNoteRequest>,
) -> impl IntoApiResponse {
    println!("Create note request: {:?}", req);
    let note = db::notes::create(state.db, user.id, req)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create note: {:?}", e);
            RestError::Database(e)
        });

    match note {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn create_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create note")
        .description("Creates a new note with the provided content")
        .tag("Note")
        .response_with::<200, Json<Note>, _>(|res| {
            res.description("Note created successfully").example(Note {
                id: 1,
                content: "This is the content of my note".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                user_id: 1,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                target_date: chrono::Utc::now().date_naive(),
            })
        })
        .response_with::<500, (), _>(|res| {
            res.description("Database error occurred while creating the note")
        })
}

pub async fn post_search(
    State(state): State<AppState>,
    Json(params): Json<NoteSearchRequest>,
) -> impl IntoApiResponse {
    let items = db::notes::search(state.db, params).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    match items {
        Ok(items) => Json(items).into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn post_search_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Search notes")
        .description("Search for notes using optional filters for text content, date, and tags")
        .tag("Note")
        .response_with::<200, Json<Vec<Note>>, _>(|res| {
            res.description("Notes matching search criteria")
                .example(vec![
                    Note {
                        id: 1,
                        content: "Discussion about Q1 goals".to_string(),
                        tags: vec!["tag1".to_string(), "tag2".to_string()],
                        user_id: 1,
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                        target_date: chrono::Utc::now().date_naive(),
                    },
                    Note {
                        id: 2,
                        content: "Status update from meeting".to_string(),
                        tags: vec!["tag1".to_string(), "tag2".to_string()],
                        user_id: 2,
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                        target_date: chrono::Utc::now().date_naive(),
                    },
                ])
        })
        .response_with::<500, (), _>(|res| {
            res.description("Database error occurred while searching notes")
        })
}
