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
struct Create {
    body: String,
    blob_ids: Vec<String>,
}

#[post("/tweets")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<()>> {
    database::tweets::create(&pool, form.body.clone(), form.blob_ids.clone())
        .await
        .map(Json)
        .map_err(Into::into)
}
