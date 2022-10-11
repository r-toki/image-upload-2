use crate::{
    lib::error::Result,
    models::command::{attachment::Attachment, tweet::Tweet},
};

use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct Create {
    body: String,
    blob_ids: Vec<String>,
}

#[post("/tweets")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;

    let tweet = Tweet::new(form.body.clone())?;
    tweet.insert(&mut tx).await?;

    for blob_id in form.blob_ids.clone() {
        let attachment =
            Attachment::new("tweets".into(), "images".into(), tweet.id.clone(), blob_id);
        attachment.insert(&mut tx).await?;
    }

    tx.commit().await?;
    Ok(Json(()))
}
