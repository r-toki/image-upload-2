use chrono::{DateTime, Utc};
use sqlx::{query, PgExecutor};
use validator::Validate;

use super::lib::{date_time::get_current_date_time, id::get_new_id};

#[derive(Debug, Validate)]
pub struct Tweet {
    pub id: String,
    #[validate(length(min = 1, max = 140))]
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Tweet {
    pub fn new(body: String) -> anyhow::Result<Self> {
        let tweet = Self {
            id: get_new_id(),
            body,
            created_at: get_current_date_time(),
        };
        tweet.validate()?;
        Ok(tweet)
    }

    pub async fn insert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into tweets (id, body, created_at)
values ($1, $2, $3)
        "#,
            self.id,
            self.body,
            self.created_at
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}
