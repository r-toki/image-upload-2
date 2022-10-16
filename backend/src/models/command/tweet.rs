use crate::models::lib::{get_current_date_time, get_new_id};

use chrono::{DateTime, Utc};
use sqlx::{query, PgPool};
use validator::Validate;

#[derive(Debug, Validate)]
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

    pub async fn store(&self, pool: &PgPool) -> anyhow::Result<()> {
        let mut tx = pool.begin().await?;

        query!(
            r#"
insert into tweets (id, body, created_at, updated_at)
values ($1, $2, $3, $4)
on conflict (id)
do update
set body = $2, created_at = $3, updated_at = $4
            "#,
            self.id,
            self.body,
            self.created_at,
            self.updated_at
        )
        .execute(&mut tx)
        .await?;

        let prev_blob_ids: Vec<String> = query!(
            r#"
select blob_id from attachments
where record_type = 'tweets'
and record_name = 'images'
and record_id = $1
            "#,
            self.id
        )
        .fetch_all(&mut tx)
        .await?
        .into_iter()
        .map(|r| r.blob_id)
        .collect();

        let added_blob_ids: Vec<String> = self
            .blob_ids
            .clone()
            .into_iter()
            .filter(|blob_id| !prev_blob_ids.contains(blob_id))
            .collect();

        let removed_blob_ids: Vec<String> = prev_blob_ids
            .into_iter()
            .filter(|prev_blob_id| !self.blob_ids.contains(prev_blob_id))
            .collect();

        for added_blob_id in added_blob_ids.iter() {
            query!(
                r#"
insert into attachments (id, record_type, record_name, record_id, blob_id)
values ($1, 'tweets', 'images', $2, $3)
                "#,
                get_new_id(),
                self.id,
                added_blob_id
            )
            .execute(&mut tx)
            .await?;
        }

        for removed_blob_id in removed_blob_ids.iter() {
            query!(
                r#"
delete from blobs
where id = $1
                "#,
                removed_blob_id
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn delete(&self, pool: &PgPool) -> anyhow::Result<()> {
        let mut tx = pool.begin().await?;

        query!(
            r#"
delete from tweets
where id = $1
            "#,
            self.id
        )
        .execute(&mut tx)
        .await?;

        for blob_id in self.blob_ids.iter() {
            query!(
                r#"
delete from blobs
where id = $1
                "#,
                blob_id
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
