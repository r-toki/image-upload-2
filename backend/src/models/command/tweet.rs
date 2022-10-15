use crate::models::{
    lib::{get_current_date_time, get_new_id},
    table,
};

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct Tweet {
    pub id: String,
    #[validate(length(min = 1, max = 140))]
    pub body: String,
    pub created_at: DateTime<Utc>,
    #[validate(length(max = 3))]
    pub blob_ids: Vec<String>,
}

impl Tweet {
    pub fn new(body: String, blob_ids: Vec<String>) -> anyhow::Result<Self> {
        let tweet = Self {
            id: get_new_id(),
            body,
            created_at: get_current_date_time(),
            blob_ids,
        };
        tweet.validate()?;
        Ok(tweet)
    }

    pub async fn upsert(&self, pool: &PgPool) -> anyhow::Result<()> {
        let mut tx = pool.begin().await?;

        let tweet = self.to_owned();

        let t_tweet: table::Tweet = tweet.clone().into();
        let t_attachments: Vec<table::Attachment> = tweet.clone().into();

        t_tweet.upsert(&mut tx).await?;
        for t_attachment in t_attachments.iter() {
            t_attachment.upsert(&mut tx).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}

impl From<Tweet> for table::Tweet {
    fn from(tweet: Tweet) -> Self {
        Self {
            id: tweet.id,
            body: tweet.body,
            created_at: tweet.created_at,
        }
    }
}

impl From<Tweet> for Vec<table::Attachment> {
    fn from(tweet: Tweet) -> Self {
        tweet
            .blob_ids
            .iter()
            .map(|blob_id| {
                table::Attachment::new(
                    get_new_id(),
                    "tweets".into(),
                    "images".into(),
                    tweet.id.clone(),
                    blob_id.clone(),
                )
            })
            .collect()
    }
}
