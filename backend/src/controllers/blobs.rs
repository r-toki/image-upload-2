use crate::models::table;
use crate::{lib::error::Result, models::query};

use actix_web::{
    get, post,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(show);
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

#[get("/blobs/{blob_id}")]
async fn show(pool: Data<PgPool>, path: Path<String>) -> Result<Json<query::Blob>> {
    let blob_id = path.into_inner();
    query::find_blob(&**pool, blob_id)
        .await
        .map(Json)
        .map_err(Into::into)
}
