use super::lib::error::Result;
use crate::model::storage::{self, Metadata, NewBlob};

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

#[post("/storages")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<String>> {
    let bytes = base64::decode(form.encoded_bytes.clone())?;
    let new_blob = NewBlob::new(bytes, Some(form.metadata.clone()));
    let blob_id = storage::insert(&pool, new_blob).await?;
    Ok(Json(blob_id))
}
