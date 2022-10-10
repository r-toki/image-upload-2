use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

#[derive(new, Debug)]
pub struct StorageIdentifier {
    pub record_type: String,
    pub record_name: String,
    pub record_id: String,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct NewBlob {
    pub bytes: Vec<u8>,
    pub metadata: Option<Metadata>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Blob {
    pub id: i32,
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
    pub blob_id: Option<i32>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: i32,
    pub record_type: Option<String>,
    pub record_name: Option<String>,
    pub record_id: Option<String>,
    pub blob_id: Option<i32>,
}

async fn find(pool: PgPool, storage_identifier: StorageIdentifier) -> anyhow::Result<Blob> {
    todo!()
}

async fn insert(
    pool: PgPool,
    storage_identifier: StorageIdentifier,
    new_blob: NewBlob,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let metadata = match new_blob.metadata {
        Some(metadata) => Some(serde_json::to_string(&metadata)?),
        None => None,
    };

    let blob = query!(
        r#"
insert into blobs (bytes, metadata, created_at)
values ($1, $2, current_timestamp)
returning id
        "#,
        new_blob.bytes,
        metadata,
    )
    .fetch_one(&mut tx)
    .await?;

    let new_attachment = NewAttachment::new(
        Some(storage_identifier.record_type),
        Some(storage_identifier.record_name),
        Some(storage_identifier.record_id),
        Some(blob.id),
    );

    query!(
        r#"
insert into attachments (record_type, record_name, record_id, blob_id)
values ($1, $2, $3, $4)
        "#,
        new_attachment.record_type,
        new_attachment.record_name,
        new_attachment.record_id,
        new_attachment.blob_id
    )
    .execute(&mut tx)
    .await?;

    Ok(())
}

async fn delete(pool: PgPool, storage_identifier: StorageIdentifier) -> anyhow::Result<()> {
    todo!()
}
