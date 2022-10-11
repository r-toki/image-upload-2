use sqlx::{query, PgExecutor};

use super::lib::id::get_new_id;

#[derive(Debug)]
pub struct Attachment {
    pub id: String,
    pub record_type: String,
    pub record_name: String,
    pub record_id: String,
    pub blob_id: String,
}

impl Attachment {
    pub fn new(
        record_type: String,
        record_name: String,
        record_id: String,
        blob_id: String,
    ) -> Self {
        Self {
            id: get_new_id(),
            record_type,
            record_name,
            record_id,
            blob_id,
        }
    }

    pub async fn insert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into attachments (id, record_type, record_name, record_id, blob_id)
values ($1, $2, $3, $4, $5)
            "#,
            self.id,
            self.record_type,
            self.record_name,
            self.record_id,
            self.blob_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
