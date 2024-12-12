use aide::axum::IntoApiResponse;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    db::repository,
    errors::{RestError, RestResult},
    model::{repository::Repository, user::User},
    state::AppState,
};

pub async fn get_all(State(state): State<AppState>) -> impl IntoApiResponse {
    let items = repository::get_all(state.db).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    if let Err(e) = items {
        return e.into_response();
    }

    Json(items.unwrap()).into_response()
}

pub async fn get_all_by_owner(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoApiResponse {
    let items = repository::get_all_by_user(state.db, user.id)
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

pub async fn get_by_id(Path(id): Path<i64>, State(state): State<AppState>) -> impl IntoApiResponse {
    let item = repository::get_by_id(state.db, id).await.map_err(|e| {
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
