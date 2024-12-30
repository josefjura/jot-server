use std::str::FromStr;

use crate::{errors::DateFilterError, util::DateTimeWrapper, util::DateWrapper};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub tags: Vec<String>,
    pub user_id: i64,
    #[schemars(with = "DateTimeWrapper")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schemars(with = "DateTimeWrapper")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[schemars(with = "DateWrapper")]
    pub target_date: NaiveDate,
}

#[derive(Debug, FromRow)]
pub struct NoteEntity {
    pub id: i64,
    pub content: String,
    pub tags: String,
    pub user_id: i64,
    pub target_date: NaiveDate,
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

        let target_date = val.target_date;

        Ok(Note {
            id: val.id,
            content: val.content,
            tags: val.tags.split(",").map(|s| s.to_string()).collect(),
            user_id: val.user_id,
            target_date,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateNoteRequest {
    pub content: String,
    pub tags: Vec<String>,
    #[schemars(with = "DateWrapper")]
    pub target_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteManyRequest {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NoteSearchRequest {
    // Optional search term
    pub term: Option<String>,
    // List of tags to filter by
    pub tag: Vec<String>,
    // Optional start date for filtering
    #[serde(default)]
    pub target_date: Option<DateFilter>,
    // Optional start date for filtering
    #[serde(default)]
    pub created_at: Option<DateFilter>,
    // Optional start date for filtering
    #[serde(default)]
    pub updated_at: Option<DateFilter>,
    // Maximum number of results to return
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateFilter {
    #[schemars(with = "DateWrapper")]
    Single(NaiveDate),
    Range {
        #[schemars(with = "DateWrapper")]
        from: Option<NaiveDate>,
        #[schemars(with = "DateWrapper")]
        until: Option<NaiveDate>,
    },
}
