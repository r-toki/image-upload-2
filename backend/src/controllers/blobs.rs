use crate::lib::error::Result;
use crate::models::command;

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
#[serde(rename_all = "camelCase")]
struct Create {
    encoded_bytes: String,
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    content_type: String,
}

#[post("/blobs")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<String>> {
    let blob = command::Blob::new(
        form.encoded_bytes.clone(),
        form.metadata.content_type.clone(),
    )?;
    blob.upsert(&**pool).await?;
    Ok(Json(blob.id))
}
