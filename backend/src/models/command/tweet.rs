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
}

impl Tweet {
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

        tx.commit().await?;

        Ok(())
    }
}
