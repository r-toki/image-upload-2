use crate::lib::error::Result;
use crate::models::table;

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
    content_type: String,
}

#[post("/blobs")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<String>> {
    let blob = table::Blob::create(form.encoded_bytes.clone(), form.content_type.clone())?;
    table::Blob::store(&**pool, &blob).await?;
    Ok(Json(blob.id))
}
