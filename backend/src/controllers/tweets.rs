use crate::{lib::error::Result, models::command};

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
    body: String,
    blob_ids: Vec<String>,
}

#[post("/tweets")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> Result<Json<()>> {
    let tweet = command::Tweet::create(form.body.clone(), form.blob_ids.clone())?;
    tweet.store(&**pool).await?;
    Ok(Json(()))
}
