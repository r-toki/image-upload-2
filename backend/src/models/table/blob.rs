use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgExecutor};

#[derive(new, Debug)]
pub struct Blob {
    pub id: String,
    pub bytes: Vec<u8>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub content_type: String,
}

impl TryFrom<Metadata> for serde_json::Value {
    type Error = anyhow::Error;
    fn try_from(metadata: Metadata) -> Result<Self, Self::Error> {
        serde_json::to_value(metadata).map_err(Into::into)
    }
}

impl Blob {
    pub async fn upsert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into blobs (id, bytes, metadata, created_at)
values ($1, $2, $3, $4)
on conflict (id)
do update
set bytes = $2, metadata = $3, created_at = $4
            "#,
            self.id,
            self.bytes,
            self.metadata,
            self.created_at
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn delete(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
delete from blobs
where id = $1
            "#,
            self.id
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn find(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<Option<Blob>> {
        query_as!(
            Blob,
            r#"
select * from blobs
where id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
