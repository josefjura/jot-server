use sqlx::SqlitePool;

use crate::{
    errors::AuthError,
    jwt::verify_password,
    model::user::{User, UserEntity},
};

pub async fn check_email_password(
    email: String,
    password: String,
    db: &SqlitePool,
) -> Result<User, AuthError> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
				SELECT id, name, email, password
				FROM users
				WHERE name = ?
			"#,
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|_| AuthError::DatabaseError)?;

    if user.is_none() {
        Err(AuthError::UserNotFound)?
    }

    let user = user.unwrap();

    if verify_password(&password, &user.password) {
        Ok(user.into())
    } else {
        Err(AuthError::PasswordIncorrect)?
    }
}
