use chrono::{DateTime, Utc};
use derive_new::new;
use serde::Serialize;
use sqlx::{query, PgPool};

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    pub id: String,
    pub body: String,
    pub blob_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_tweets(pool: &PgPool) -> anyhow::Result<Vec<Tweet>> {
    let raw_tweets = query!(
        r#"
select t.id id, t.body body, array_agg(a.blob_id) blob_ids, t.created_at created_at, t.updated_at updated_at
from tweets t
join attachments a
on a.record_type = 'tweets'
and a.record_name = 'images'
and a.record_id = t.id
group by t.id
            "#
    )
    .fetch_all(pool)
    .await?;

    let tweets = raw_tweets
        .into_iter()
        .map(|r_t| {
            Tweet::new(
                r_t.id,
                r_t.body,
                r_t.blob_ids.unwrap_or(vec![]),
                r_t.created_at,
                r_t.updated_at,
            )
        })
        .collect();

    Ok(tweets)
}
