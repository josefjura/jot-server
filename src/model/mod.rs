use schemars::JsonSchema;
use serde::Deserialize;

pub mod note;
pub mod repository;
pub mod user;

/// Struct for holding data from the user login form.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}