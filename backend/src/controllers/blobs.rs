use crate::{
    lib::error::Result,
    models::command::blob::{Blob, Metadata},
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
#[serde(rename_all = "camelCase")]
struct Create {
    encoded_bytes: String,
    metadata: CreateMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateMetadata {
    content_type: String,
}

#[post("/blobs")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<String>> {
    let blob = Blob::new(
        form.encoded_bytes.clone(),
        Metadata::new(form.metadata.content_type.clone()),
    )?;
    blob.insert(&**pool).await?;
    Ok(Json(blob.id))
}
