use derive_new::new;
use sqlx::{query, query_as, PgExecutor};

#[derive(new, Debug)]
pub struct Attachment {
    pub id: String,
    pub record_type: String,
    pub record_name: String,
    pub record_id: String,
    pub blob_id: String,
}

impl Attachment {
    pub async fn upsert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into attachments (id, record_type, record_name, record_id, blob_id)
values ($1, $2, $3, $4, $5)
on conflict (id)
do update
set record_type = $2, record_name = $3, record_id = $4, blob_id = $5
            "#,
            self.id,
            self.record_type,
            self.record_name,
            self.record_id,
            self.blob_id
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn delete(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
delete from attachments
where id = $1
            "#,
            self.id
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn find(
        executor: impl PgExecutor<'_>,
        id: String,
    ) -> anyhow::Result<Option<Attachment>> {
        query_as!(
            Attachment,
            r#"
select * from attachments
where id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
