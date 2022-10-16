use crate::models::lib::{get_current_date_time, get_new_id};

use chrono::{DateTime, Utc};
use sqlx::{query, PgPool};

#[derive(Debug)]
pub struct Blob {
    pub id: String,
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
}

impl Blob {
    pub fn create(encoded_bytes: String, content_type: String) -> anyhow::Result<Self> {
        let blob = Self {
            id: get_new_id(),
            bytes: base64::decode(encoded_bytes)?,
            content_type,
            created_at: get_current_date_time(),
        };
        Ok(blob)
    }

    pub async fn store(&self, pool: &PgPool) -> anyhow::Result<()> {
        query!(
            r#"
insert into blobs (id, bytes, content_type, created_at)
values ($1, $2, $3, $4)
            "#,
            self.id,
            self.bytes,
            self.content_type,
            self.created_at
        )
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
