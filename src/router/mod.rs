use std::sync::Arc;

use aide::{
    axum::{
        routing::{get, get_with, post_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::{OpenApi, Tag},
    redoc::Redoc,
    transform::{TransformOpenApi, TransformOperation},
};
use auth::{login_post, logout_post};
use axum::{middleware, response::IntoResponse, Extension, Json, Router};
use axum_extra::response::Html;
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::{middleware::auth_middleware, model::repository::Repository, state::AppState};

pub mod auth;
pub mod health;
pub mod repository;

// Corrected function to configure the OpenAPI documentation
fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Jot server")
        .version(env!("CARGO_PKG_VERSION"))
        .summary("API documentation for Jot server")
        .description(include_str!("../../README.md"))
        .tag(Tag {
            name: "Health".into(),
            description: Some("Health check endpoints".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Repository".into(),
            description: Some("Repository managenement endpoints".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Authentication".into(),
            description: Some("Authentication endpoints".into()),
            ..Default::default()
        })
}

pub fn setup_router(db: SqlitePool, jwt_secret: &str) -> Router {
    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);
    let mut api = OpenApi::default();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);
    let app_state = AppState::new(db, jwt_secret);
    aide::gen::infer_responses(true);
    let router = ApiRouter::new()
        .api_route("/health", get_with(health::ping, ping_docs))
        .api_route(
            "/health/auth",
            get_with(health::auth_ping, auth_ping_docs).route_layer(
                middleware::from_fn_with_state(app_state.clone(), auth_middleware),
            ),
        )
        .api_route(
            "/repository",
            get_with(repository::get_all, get_all_docs).route_layer(
                middleware::from_fn_with_state(app_state.clone(), auth_middleware),
            ),
        )
        .api_route(
            "/repository/:id",
            get_with(repository::get_by_id, get_by_id_docs).route_layer(
                middleware::from_fn_with_state(app_state.clone(), auth_middleware),
            ),
        )
        .api_route(
            "/user/repository",
            get_with(repository::get_all_by_owner, get_all_by_owner_docs).route_layer(
                middleware::from_fn_with_state(app_state.clone(), auth_middleware),
            ),
        )
        .api_route("/login", post_with(login_post, login_post_docs))
        .api_route("/logout", post_with(logout_post, logout_post_docs))
        .route("/docs.json", get(serve_docs))
        .route("/docs", get(serve_swagger))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    aide::gen::infer_responses(false);

    router
}

// Handler to serve Swagger UI
async fn serve_swagger() -> impl IntoApiResponse {
    let html = Redoc::new("/docs.json").html();
    Html(html).into_response()
}

// Handler to serve OpenAPI JSON
async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api.clone())
}

fn login_post_docs(op: TransformOperation) -> TransformOperation {
    op.description("Authenticate user and receive session token")
        .tag("Authentication")
        .response::<200, ()>()
        .response_with::<400, (), _>(|res| {
            res.description("Invalid request - missing username or password")
        })
        .response_with::<401, (), _>(|res| {
            res.description("Authentication failed - invalid credentials")
        })
}

pub fn logout_post_docs(op: TransformOperation) -> TransformOperation {
    op.description("Logout user and reject session token")
        .tag("Authentication")
        .response::<200, ()>()
}

fn get_all_docs(op: TransformOperation) -> TransformOperation {
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

fn get_by_id_docs(op: TransformOperation) -> TransformOperation {
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

fn get_all_by_owner_docs(op: TransformOperation) -> TransformOperation {
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

pub fn ping_docs(op: TransformOperation) -> TransformOperation {
    op.description("Health check endpoint")
        .tag("Health")
        .response::<200, ()>() // Simple 200 OK response with no body
}

pub fn auth_ping_docs(op: TransformOperation) -> TransformOperation {
    op.description("Health check endpoint requiring authentication")
        .tag("Health")
        .response::<200, ()>() // Simple 200 OK response with no body
        .response_with::<401, (), _>(|res| res.description("Not authenticated"))
}
