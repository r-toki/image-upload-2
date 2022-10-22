use crate::models::lib::get_new_id;

use derive_new::new;
use serde::Deserialize;
use sqlx::{query, query_as, PgExecutor};

#[derive(new, Debug, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub record_type: String,
    pub record_name: String,
    pub record_id: String,
    pub blob_id: String,
}

impl Attachment {
    pub fn create(
        record_type: String,
        record_name: String,
        record_id: String,
        blob_id: String,
    ) -> Attachment {
        Attachment::new(get_new_id(), record_type, record_name, record_id, blob_id)
    }
}

impl Attachment {
    pub async fn find(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<Attachment> {
        query_as!(
            Attachment,
            r#"
select * from attachments
where id = $1
            "#,
            id
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn store(
        executor: impl PgExecutor<'_>,
        attachment: &Attachment,
    ) -> anyhow::Result<()> {
        query!(
            r#"
insert into attachments (id, record_type, record_name, record_id, blob_id)
values ($1, $2, $3, $4, $5)
on conflict (id)
do update
set record_type = $2, record_name = $3, record_id = $4, blob_id = $5
            "#,
            attachment.id,
            attachment.record_type,
            attachment.record_name,
            attachment.record_id,
            attachment.blob_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<()> {
        query!(
            r#"
delete from attachments
where id = $1
            "#,
            id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
