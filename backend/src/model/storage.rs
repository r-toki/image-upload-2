use super::lib::get_new_id;

use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

#[derive(new, Debug, Serialize, Deserialize)]
pub struct NewBlob {
    pub bytes: Vec<u8>,
    pub metadata: Option<Metadata>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Blob {
    pub id: String,
    pub bytes: Vec<u8>,
    pub metadata: Option<Metadata>,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub content_type: String,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct NewAttachment {
    pub record_type: Option<String>,
    pub record_name: Option<String>,
    pub record_id: Option<String>,
    pub blob_id: Option<String>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: i32,
    pub record_type: Option<String>,
    pub record_name: Option<String>,
    pub record_id: Option<String>,
    pub blob_id: Option<String>,
}

async fn find(pool: PgPool, blob_id: String) -> anyhow::Result<Blob> {
    todo!()
}

async fn insert(pool: PgPool, new_blob: NewBlob) -> anyhow::Result<String> {
    let mut tx = pool.begin().await?;

    let metadata = match new_blob.metadata {
        Some(metadata) => Some(serde_json::to_string(&metadata)?),
        None => None,
    };

    let blob = query!(
        r#"
insert into blobs (id, bytes, metadata, created_at)
values ($1, $2, $3, current_timestamp)
returning id
        "#,
        get_new_id(),
        new_blob.bytes,
        metadata,
    )
    .fetch_one(&mut tx)
    .await?;

    Ok(blob.id)
}

async fn delete(pool: PgPool, blob_id: String) -> anyhow::Result<()> {
    todo!()
}
