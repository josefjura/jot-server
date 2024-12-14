use sqlx::SqlitePool;

use crate::{
    errors::DbError,
    model::note::{Note, NoteEntity},
};

pub async fn get_all(db: SqlitePool) -> Result<Vec<Note>, DbError> {
    let items = sqlx::query_as!(NoteEntity, "SELECT * FROM notes")
        .fetch_all(&db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get all repositories: {:?}", e);
            DbError::Unknown(e)
        })?;

    items
        .into_iter()
        .map(|item| item.try_into().map_err(DbError::EntityMapping))
        .collect()
}

pub async fn get_all_by_user(db: SqlitePool, id: i64) -> Result<Vec<Note>, DbError> {
    let items = sqlx::query_as!(
        NoteEntity,
        "SELECT notes.* FROM notes
				JOIN repositories ON notes.repository_id = repositories.id
				WHERE repositories.user_id = ?",
        id
    )
    .fetch_all(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        DbError::Unknown(e)
    })?;

    items
        .into_iter()
        .map(|item| {
            item.try_into().map_err(|e| {
                // Assuming e is a String from our TryFrom implementation
                tracing::error!("Failed to convert note entity: {:?}", e);
                DbError::EntityMapping(e)
            })
        })
        .collect()
}

pub async fn get_all_by_repository(db: SqlitePool, id: i64) -> Result<Vec<Note>, DbError> {
    let items = sqlx::query_as!(
        NoteEntity,
        "SELECT * FROM notes WHERE repository_id = ?",
        id
    )
    .fetch_all(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get all repositories: {:?}", e);
        DbError::Unknown(e)
    })?;

    items
        .into_iter()
        .map(|item| {
            item.try_into().map_err(|e| {
                tracing::error!("Failed to convert note entity: {:?}", e);
                DbError::EntityMapping(e)
            })
        })
        .collect()
}

pub async fn get_by_id(db: SqlitePool, id: i64) -> Result<Option<Note>, DbError> {
    let item = sqlx::query_as!(NoteEntity, "SELECT * FROM notes WHERE id = ?", id)
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get repository by id: {:?}", e);
            DbError::Unknown(e)
        })?;

    item.map(|entity| entity.try_into().map_err(DbError::EntityMapping))
        .transpose()
}
