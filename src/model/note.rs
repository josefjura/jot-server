use crate::util::DateTimeWrapper;
use chrono::{DateTime, NaiveDateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub repository_id: i64,
    #[schemars(with = "DateTimeWrapper")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schemars(with = "DateTimeWrapper")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct NoteEntity {
    pub id: i64,
    pub content: String,
    pub repository_id: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl TryFrom<NoteEntity> for Note {
    type Error = String;

    fn try_from(val: NoteEntity) -> Result<Self, String> {
        let created_at = val
            .created_at
            .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            .ok_or("created_at timestamp is missing".to_string())?;

        let updated_at = val
            .updated_at
            .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            .ok_or("updated_at timestamp is missing".to_string())?;

        Ok(Note {
            id: val.id,
            content: val.content,
            repository_id: val.repository_id,
            created_at,
            updated_at,
        })
    }
}
