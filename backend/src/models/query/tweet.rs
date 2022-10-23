use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    pub id: String,
    pub body: String,
    pub blob_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_tweets(pool: &PgPool) -> anyhow::Result<Vec<Tweet>> {
    query_as!(
        Tweet,
        r#"
select
    t.id id,
    t.body body,
    array(select a.blob_id from attachments a where a.record_type = 'tweets' and a.record_name = 'images' and a.record_id = t.id) "blob_ids!",
    t.created_at created_at,
    t.updated_at updated_at
from tweets t
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
