use crate::models::lib::{get_current_date_time, get_new_id};

use chrono::{DateTime, Utc};
use derive_new::new;
use sqlx::{query, query_as, PgExecutor};

#[derive(new, Debug)]
pub struct Blob {
    pub id: String,
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
}

impl Blob {
    pub fn create(encoded_bytes: String, content_type: String) -> anyhow::Result<Blob> {
        let blob = Blob::new(
            get_new_id(),
            base64::decode(encoded_bytes)?,
            content_type,
            get_current_date_time(),
        );
        Ok(blob)
    }
}

impl Blob {
    pub async fn find(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<Blob> {
        query_as!(
            Blob,
            r#"
select * from blobs
where id = $1
            "#,
            id
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn store(executor: impl PgExecutor<'_>, blob: &Blob) -> anyhow::Result<()> {
        query!(
            r#"
insert into blobs (id, bytes, content_type, created_at)
values ($1, $2, $3, $4)
            "#,
            blob.id,
            blob.bytes,
            blob.content_type,
            blob.created_at
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<()> {
        query!(
            r#"
delete from blobs
where id = $1
            "#,
            id
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
