use crate::lib::utils::vec_diff;
use crate::models::lib::{get_current_date_time, get_new_id};
use crate::models::table::Attachment;

use chrono::{DateTime, Utc};
use derive_new::new;
use sqlx::{query, PgExecutor, PgPool};
use validator::Validate;

#[derive(new, Debug, Validate)]
pub struct Tweet {
    pub id: String,
    #[validate(length(min = 1, max = 140))]
    pub body: String,
    #[validate(length(max = 3))]
    pub blob_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tweet {
    pub fn create(body: String, blob_ids: Vec<String>) -> anyhow::Result<Self> {
        let now = get_current_date_time();
        let tweet = Self {
            id: get_new_id(),
            body,
            blob_ids,
            created_at: now,
            updated_at: now,
        };
        tweet.validate()?;
        Ok(tweet)
    }

    pub fn update(&mut self, body: String, blob_ids: Vec<String>) -> anyhow::Result<()> {
        self.body = body;
        self.blob_ids = blob_ids;
        self.updated_at = get_current_date_time();
        self.validate()?;
        Ok(())
    }
}

impl Tweet {
    pub async fn find(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<Tweet> {
        let raw_tweet = query!(
            r#"
select t.id id, t.body body, array_agg(a.blob_id) blob_ids, t.created_at created_at, t.updated_at updated_at
from tweets t
join attachments a
on a.record_type = 'tweets'
and a.record_name = 'images'
and a.record_id = t.id
where t.id = $1
group by t.id
            "#,
            id
        )
        .fetch_one(executor)
        .await?;

        let tweet = Tweet::new(
            raw_tweet.id,
            raw_tweet.body,
            raw_tweet.blob_ids.unwrap_or(vec![]),
            raw_tweet.created_at,
            raw_tweet.updated_at,
        );

        Ok(tweet)
    }

    pub async fn store(pool: &PgPool, tweet: &Tweet) -> anyhow::Result<()> {
        let mut tx = pool.begin().await?;

        query!(
            r#"
insert into tweets (id, body, created_at, updated_at)
values ($1, $2, $3, $4)
on conflict (id)
do update
set body = $2, created_at = $3, updated_at = $4
            "#,
            tweet.id,
            tweet.body,
            tweet.created_at,
            tweet.updated_at
        )
        .execute(&mut tx)
        .await?;

        let prev_tweet = Tweet::find(&mut tx, tweet.id.clone()).await?;

        let added_blob_ids = vec_diff(tweet.blob_ids.clone(), prev_tweet.blob_ids.clone());
        let removed_blob_ids = vec_diff(prev_tweet.blob_ids.clone(), tweet.blob_ids.clone());

        for added_blob_id in added_blob_ids {
            Attachment::store(
                &mut tx,
                &Attachment::create(
                    "tweets".into(),
                    "images".into(),
                    tweet.id.clone(),
                    added_blob_id,
                ),
            )
            .await?;
        }
        for removed_blob_id in removed_blob_ids {
            Attachment::delete(&mut tx, removed_blob_id).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: String) -> anyhow::Result<()> {
        let mut tx = pool.begin().await?;

        query!(
            r#"
delete from tweets
where id = $1
            "#,
            id
        )
        .execute(&mut tx)
        .await?;

        let tweet = Tweet::find(&mut tx, id).await?;

        for blob_id in tweet.blob_ids {
            Attachment::delete(&mut tx, blob_id).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
