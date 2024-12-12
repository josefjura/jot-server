use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub repository_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct NoteEntity {
    pub id: i64,
    pub content: String,
    pub repository_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<NoteEntity> for Note {
    fn from(val: NoteEntity) -> Self {
        Note {
            id: val.id,
            content: val.content,
            repository_id: val.repository_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
