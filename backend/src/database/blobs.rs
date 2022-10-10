use super::lib::id::get_new_id;

use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

#[derive(new, Debug, Serialize, Deserialize)]
struct Metadata {
    content_type: String,
}

pub async fn create(
    pool: &PgPool,
    encoded_bytes: String,
    content_type: String,
) -> anyhow::Result<String> {
    let bytes = base64::decode(&encoded_bytes)?;

    let metadata = Metadata::new(content_type);
    let metadata = serde_json::to_string(&metadata)?;

    let blob = query!(
        r#"
insert into blobs (id, bytes, metadata, created_at)
values ($1, $2, $3, current_timestamp)
returning id
        "#,
        get_new_id(),
        bytes,
        metadata,
    )
    .fetch_one(pool)
    .await?;

    Ok(blob.id)
}
