use super::lib::{date_time::get_current_date_time, id::get_new_id};

use chrono::{DateTime, Utc};
use derive_new::new;
use serde::Serialize;
use sqlx::{query, PgExecutor};

#[derive(Debug)]
pub struct Blob {
    pub id: String,
    pub bytes: Vec<u8>,
    pub metadata: String,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug, Serialize)]
pub struct Metadata {
    pub content_type: String,
}

impl Blob {
    pub fn new(encoded_bytes: String, metadata: Metadata) -> anyhow::Result<Self> {
        let bytes = base64::decode(encoded_bytes)?;
        let metadata = serde_json::to_string(&metadata)?;
        let blob = Self {
            id: get_new_id(),
            bytes,
            metadata,
            created_at: get_current_date_time(),
        };
        Ok(blob)
    }

    pub async fn insert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into blobs (id, bytes, metadata, created_at)
values ($1, $2, $3, $4)
            "#,
            self.id,
            self.bytes,
            self.metadata,
            self.created_at
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}
