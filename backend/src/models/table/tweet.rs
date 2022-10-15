use chrono::{DateTime, Utc};
use derive_new::new;
use sqlx::{query, query_as, PgExecutor};

#[derive(new, Debug)]
pub struct Tweet {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Tweet {
    pub async fn upsert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into tweets (id, body, created_at)
values ($1, $2, $3)
on conflict (id)
do update
set body = $2, created_at = $3
            "#,
            self.id,
            self.body,
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
delete from tweets
where id = $1
            "#,
            self.id
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn find(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<Option<Tweet>> {
        query_as!(
            Tweet,
            r#"
select * from tweets
where id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
