use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug)]
pub struct StorageIdentifier {
    pub record_type: String,
    pub record_name: String,
    pub record_id: String,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct NewBlob {
    pub bytes: Vec<u8>,
    pub metadata: Option<String>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Blob {
    pub id: i32,
    pub bytes: Vec<u8>,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
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

async fn find(storage_identifier: StorageIdentifier) -> anyhow::Result<Blob> {
    todo!()
}

async fn insert(storage_identifier: StorageIdentifier, new_blob: NewBlob) -> anyhow::Result<()> {
    todo!()
}

async fn delete(storage_identifier: StorageIdentifier) -> anyhow::Result<()> {
    todo!()
}
