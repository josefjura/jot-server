use aide::{
    axum::{routing::post_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Json,
};

use jsonwebtoken::{encode, EncodingKey, Header};
use tower_sessions::cookie::{time::Duration, Cookie, SameSite};
use tracing::error;

use crate::{
    db::auth::check_email_password,
    errors::RestError,
    jwt::TokenClaims,
    model::{auth::LoginResponse, LoginUserSchema},
    state::AppState,
};

pub fn auth_routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/login", post_with(login_post, login_post_docs))
        .api_route("/logout", post_with(logout_post, logout_post_docs))
}

pub async fn login_post(
    State(state): State<AppState>,
    Json(form_data): Json<LoginUserSchema>,
) -> impl IntoApiResponse {
    let result = check_email_password(
        form_data.username.clone(),
        form_data.password.clone(),
        &state.db,
    )
    .await;

    if form_data.username.is_empty() || form_data.password.is_empty() {
        return RestError::InvalidInput("Username and password are required".to_string())
            .into_response();
    }

    if let Err(err) = &result {
        error!("{}", err);
        return RestError::Authorization(err.clone()).into_response();
    }

    let user_id = result.unwrap().id;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::days(7)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(Duration::days(5))
        .same_site(SameSite::Lax)
        .http_only(true);

    let headers = AppendHeaders([(SET_COOKIE, cookie.to_string())]);

    (headers, Json(LoginResponse { token })).into_response()
}

pub fn login_post_docs(op: TransformOperation) -> TransformOperation {
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

pub async fn logout_post() -> impl IntoApiResponse {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let headers = AppendHeaders([(SET_COOKIE, cookie.to_string())]);

    (headers, StatusCode::OK)
}

pub fn logout_post_docs(op: TransformOperation) -> TransformOperation {
    op.description("Logout user and reject session token")
        .tag("Authentication")
        .response::<200, ()>()
}
