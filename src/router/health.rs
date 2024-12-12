use aide::axum::IntoApiResponse;
use axum::http::StatusCode;

pub async fn ping() -> impl IntoApiResponse {
    StatusCode::OK
}

pub async fn auth_ping() -> impl IntoApiResponse {
    StatusCode::OK
}
