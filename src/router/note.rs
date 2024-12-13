use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;

use crate::{
    db,
    errors::RestError,
    model::{note::Note, user::User},
    state::AppState,
};

use super::with_auth_middleware;

pub fn note_routes(app_state: AppState) -> ApiRouter<AppState> {
    let router = ApiRouter::new()
        .api_route("/note", get_with(get_all, get_all_docs))
        .api_route("/note/:id", get_with(get_by_id, get_by_id_docs))
        .api_route(
            "/user/note",
            get_with(get_all_by_owner, get_all_by_owner_docs),
        )
        .api_route(
            "/repository/:id/note",
            get_with(get_all_by_repository, get_all_by_repository_docs),
        );

    with_auth_middleware(router, app_state)
}

pub async fn get_all(State(state): State<AppState>) -> impl IntoApiResponse {
    let items = db::notes::get_all(state.db).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    if let Err(e) = items {
        return e.into_response();
    }

    Json(items.unwrap()).into_response()
}

pub fn get_all_docs(op: TransformOperation) -> TransformOperation {
    op.description("Retrieve all existing notes")
        .tag("Note")
        .response_with::<200, Json<Vec<Note>>, _>(|res| {
            res.example(vec![
                Note {
                    id: 1,
                    content: "Some note".to_string(),
                    repository_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                Note {
                    id: 1,
                    content: "Some other note".to_string(),
                    repository_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
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

    if let Err(e) = items {
        return e.into_response();
    }

    Json(items.unwrap()).into_response()
}

pub fn get_all_by_owner_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Retrieve user repositories")
        .description("Retrieve all notes owned by the authenticated user")
        .tag("User")
        .response_with::<200, Json<Vec<Note>>, _>(|res| {
            res.example(vec![
                Note {
                    id: 1,
                    content: "Some note".to_string(),
                    repository_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                Note {
                    id: 1,
                    content: "Some other note".to_string(),
                    repository_id: 2,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            ])
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn get_by_id(Path(id): Path<i64>, State(state): State<AppState>) -> impl IntoApiResponse {
    let item = db::notes::get_by_id(state.db, id).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    if let Err(e) = item {
        return e.into_response();
    }

    let item = item.unwrap();

    if item.is_none() {
        return RestError::NotFound.into_response();
    }

    Json(item).into_response()
}

pub fn get_by_id_docs(op: TransformOperation) -> TransformOperation {
    op.description("Retrieve a note by its ID")
        .tag("Note")
        .response_with::<200, Json<Note>, _>(|res| {
            res.example(Note {
                id: 1,
                content: "Some note".to_string(),
                repository_id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn get_all_by_repository(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoApiResponse {
    let items = db::notes::get_all_by_repository(state.db, user.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get all repositories: {:?}", e);
            RestError::Database(e)
        });

    if let Err(e) = items {
        return e.into_response();
    }

    Json(items.unwrap()).into_response()
}

pub fn get_all_by_repository_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Retrieve all by repository")
        .description("Retrieve all notes in specified repository")
        .tag("Repository")
        .response_with::<200, Json<Vec<Note>>, _>(|res| {
            res.example(vec![
                Note {
                    id: 1,
                    content: "Some note".to_string(),
                    repository_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                Note {
                    id: 1,
                    content: "Some other note".to_string(),
                    repository_id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            ])
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}
