use super::lib::id::get_new_id;

use sqlx::{query, PgPool};

pub async fn create(pool: &PgPool, body: String, blob_ids: Vec<String>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let tweet = query!(
        r#"
insert into tweets (id, body, created_at)
values ($1, $2, current_timestamp)
returning id
        "#,
        get_new_id(),
        body
    )
    .fetch_one(&mut tx)
    .await?;

    for blob_id in blob_ids {
        query!(
            r#"
insert into attachments (id, record_type, record_name, record_id, blob_id)
values ($1, 'tweets', 'images', $2, $3)
            "#,
            get_new_id(),
            tweet.id,
            blob_id
        )
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
