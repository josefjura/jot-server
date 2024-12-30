use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::{
    errors::{self, DbError},
    model::note::{CreateNoteRequest, DateFilter, Note, NoteEntity, NoteSearchRequest},
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
				WHERE user_id = ?",
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

pub async fn create(
    db: SqlitePool,
    user_id: i64,
    note: CreateNoteRequest,
) -> Result<Note, DbError> {
    let tag_value = note.tags.join(",");

    let id = sqlx::query!(
        r#"
			INSERT INTO notes (content, tags, user_id, target_date)
			VALUES (?, ?, ?, ?)
		"#,
        note.content,
        tag_value,
        user_id,
        note.target_date
    )
    .execute(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create note: {:?}", e);
        DbError::Unknown(e)
    })?
    .last_insert_rowid();

    let created = get_by_id(db, id)
        .await?
        .ok_or(DbError::UnableToCreate("Note not found".to_string()))?;

    Ok(created)
}

pub async fn search(db: SqlitePool, params: NoteSearchRequest) -> Result<Vec<Note>, DbError> {
    let mut query_builder = QueryBuilder::new("SELECT * FROM notes WHERE 1=1");

    // Add search term if present
    if let Some(term) = params.term {
        query_builder
            .push(" AND content LIKE ")
            .push_bind(format!("%{}%", term));
    }

    // Add tags (if any)
    for tag in params.tag {
        query_builder
            .push(" AND tags LIKE ")
            .push_bind(format!("%{}%", tag));
    }

    // Add date filters
    if let Some(date_filter) = params.target_date {
        date_filter.apply_to_query(&mut query_builder, "target_date");
    }

    if let Some(date_filter) = params.created_at {
        date_filter.apply_to_query(&mut query_builder, "created_at");
    }

    if let Some(date_filter) = params.updated_at {
        date_filter.apply_to_query(&mut query_builder, "updated_at");
    }

    // Add ordering
    query_builder.push(" ORDER BY created_at DESC");

    // Add limit if present
    if let Some(limit) = params.limit {
        query_builder.push(" LIMIT ").push_bind(limit);
    }

    // Build and execute the query
    let items = query_builder
        .build_query_as::<NoteEntity>()
        .fetch_all(&db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to search notes: {:?}", e);
            DbError::Unknown(e)
        })?;

    // Convert entities to domain objects
    items
        .into_iter()
        .map(|item| item.try_into().map_err(DbError::EntityMapping))
        .collect()
}

impl DateFilter {
    pub fn apply_to_query(&self, query: &mut QueryBuilder<Sqlite>, field: &str) {
        match self {
            DateFilter::Single(date) => {
                let next_day = *date + chrono::Duration::days(1);
                query
                    .push(" AND ")
                    .push(field)
                    .push(" >= ")
                    .push_bind(*date)
                    .push(" AND ")
                    .push(field)
                    .push(" < ")
                    .push_bind(next_day);
            }
            DateFilter::Range { from, until } => {
                if let Some(from_date) = from {
                    query
                        .push(" AND ")
                        .push(field)
                        .push(" >= ")
                        .push_bind(*from_date);
                }

                if let Some(until_date) = until {
                    query
                        .push(" AND ")
                        .push(field)
                        .push(" <= ")
                        .push_bind(*until_date);
                }
            }
        }
    }
}

pub async fn delete_many(db: SqlitePool, ids: &[i64], user_id: i64) -> Result<(), DbError> {
    let params = serde_json::to_string(&ids).map_err(errors::DbError::JsonError)?;

    sqlx::query!(
        "DELETE FROM notes WHERE id IN (SELECT value FROM json_each(?)) and user_id = ?",
        params,
        user_id
    )
    .execute(&db)
    .await?;

    Ok(())
}

pub async fn delete(db: SqlitePool, id: i64, user_id: i64) -> Result<(), DbError> {
    sqlx::query!(
        "DELETE FROM notes WHERE id = ? and user_id = ?",
        id,
        user_id
    )
    .execute(&db)
    .await?;

    Ok(())
}
