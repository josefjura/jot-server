use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    db,
    errors::RestError,
    model::{repository::Repository, user::User},
    state::AppState,
};

use super::with_auth_middleware;

pub fn repository_routes(app_state: AppState) -> ApiRouter<AppState> {
    let router = ApiRouter::new()
        .api_route("/repository", get_with(get_all, get_all_docs))
        .api_route("/repository/:id", get_with(get_by_id, get_by_id_docs))
        .api_route(
            "/user/repository",
            get_with(get_all_by_owner, get_all_by_owner_docs),
        );

    with_auth_middleware(router, app_state)
}

pub async fn get_all(State(state): State<AppState>) -> impl IntoApiResponse {
    let items = db::repository::get_all(state.db).await.map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        RestError::Database(e)
    });

    if let Err(e) = items {
        return e.into_response();
    }

    Json(items.unwrap()).into_response()
}

pub fn get_all_docs(op: TransformOperation) -> TransformOperation {
    op.description("Retrieve all existing repositories")
        .tag("Repository")
        .response_with::<200, Json<Vec<Repository>>, _>(|res| {
            res.example(vec![
                Repository {
                    id: 1,
                    name: "Example repo".to_string(),
                    user_id: 1,
                },
                Repository {
                    id: 2,
                    name: "Example repo".to_string(),
                    user_id: 2,
                },
            ])
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn get_all_by_owner(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoApiResponse {
    let items = db::repository::get_all_by_user(state.db, user.id)
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
    op.description("Retrieve all repositories owned by the authenticated user")
        .tag("Repository")
        .response_with::<200, Json<Vec<Repository>>, _>(|res| {
            res.example(vec![
                Repository {
                    id: 1,
                    name: "Example repo 1".to_string(),
                    user_id: 1,
                },
                Repository {
                    id: 2,
                    name: "Example repo 2".to_string(),
                    user_id: 1,
                },
            ])
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}

pub async fn get_by_id(Path(id): Path<i64>, State(state): State<AppState>) -> impl IntoApiResponse {
    let item = db::repository::get_by_id(state.db, id).await.map_err(|e| {
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
    op.description("Retrieve a repository by its ID")
        .tag("Repository")
        .response_with::<200, Json<Repository>, _>(|res| {
            res.example(Repository {
                id: 1,
                name: "Example repo 1".to_string(),
                user_id: 1,
            })
        })
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}
