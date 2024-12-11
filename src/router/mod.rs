use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use sqlx::SqlitePool;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::state::AppState;

pub fn setup_router(db: SqlitePool, jwt_secret: &str) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);
    let app_state = AppState::new(db, jwt_secret);
    Router::new()
        // .route(
        //     "/",
        //     get(index).route_layer(middleware::from_fn_with_state(
        //         app_state.clone(),
        //         auth_middleware,
        //     )),
        // )
        // .route(
        //     "/code",
        //     post(add_code).route_layer(middleware::from_fn_with_state(
        //         app_state.clone(),
        //         auth_middleware,
        //     )),
        // )
        // .route(
        //     "/code/reset",
        //     post(reset_codes).route_layer(middleware::from_fn_with_state(
        //         app_state.clone(),
        //         auth_middleware,
        //     )),
        // )
        // .route("/login", get(login).post(login_post))
        // .route("/logout", post(logout_post))
        // .route(
        //     "/change-password",
        //     get(change_password).post(change_password_post).route_layer(
        //         middleware::from_fn_with_state(app_state.clone(), auth_middleware),
        //     ),
        // )
        // .route(
        //     "/admin/user",
        //     get(get_users)
        //         .post(create_user)
        //         .route_layer(middleware::from_fn_with_state(
        //             app_state.clone(),
        //             auth_middleware,
        //         )),
        // )
        // .route(
        //     "/admin/user/:id",
        //     delete(delete_user).route_layer(middleware::from_fn_with_state(
        //         app_state.clone(),
        //         auth_middleware,
        //     )),
        // )
        // .nest_service("/assets", ServeDir::new("assets"))
        // .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
