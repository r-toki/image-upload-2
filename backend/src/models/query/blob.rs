use crate::models::table;

use chrono::{DateTime, Utc};
use derive_new::new;
use serde::Serialize;
use sqlx::PgPool;

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    src: String,
    created_at: DateTime<Utc>,
}

pub async fn find_blob(pool: &PgPool, id: String) -> anyhow::Result<Blob> {
    let blob_row = table::Blob::find(pool, id).await?;
    let encoded_bytes = base64::encode(blob_row.bytes);
    let src = vec!["data:", &blob_row.content_type, ";base64,", &encoded_bytes].join("");
    let blob = Blob::new(src, blob_row.created_at);
    Ok(blob)
}
