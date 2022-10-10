use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{database, lib::error::Result};

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
    database::blobs::create(&pool, form.encoded_bytes.clone(), form.content_type.clone())
        .await
        .map(Json)
        .map_err(Into::into)
}
