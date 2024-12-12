use sqlx::SqlitePool;

use crate::{
    errors::DbError,
    model::repository::{Repository, RepositoryEntity},
};

pub async fn get_all(db: SqlitePool) -> Result<Vec<Repository>, DbError> {
    let items = sqlx::query_as!(RepositoryEntity, "SELECT * FROM repositories")
        .fetch_all(&db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get all repositories: {:?}", e);
            DbError::Unknown(e)
        })?;

    Ok(items.into_iter().map(|i| i.into()).collect())
}

pub async fn get_all_by_user(db: SqlitePool, id: i64) -> Result<Vec<Repository>, DbError> {
    let items = sqlx::query_as!(
        RepositoryEntity,
        "SELECT * FROM repositories WHERE user_id = ?",
        id
    )
    .fetch_all(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        DbError::Unknown(e)
    })?;

    Ok(items.into_iter().map(|i| i.into()).collect())
}

pub async fn get_by_id(db: SqlitePool, id: i64) -> Result<Option<Repository>, DbError> {
    let item = sqlx::query_as!(
        RepositoryEntity,
        "SELECT * FROM repositories WHERE id = ?",
        id
    )
    .fetch_optional(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get repository by id: {:?}", e);
        DbError::Unknown(e)
    })?;

    Ok(item.map(|i| i.into()))
}
