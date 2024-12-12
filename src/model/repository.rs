use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Struct representing a note repository.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
}

#[derive(Debug)]
pub struct RepositoryEntity {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
}

impl From<RepositoryEntity> for Repository {
    fn from(val: RepositoryEntity) -> Self {
        Repository {
            id: val.id,
            name: val.name,
            user_id: val.user_id,
        }
    }
}
