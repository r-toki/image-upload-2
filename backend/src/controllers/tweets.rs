use crate::{
    lib::error::Result,
    models::{command, query},
};

use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
}

#[get("/tweets")]
async fn index(pool: Data<PgPool>) -> Result<Json<Vec<query::Tweet>>> {
    let tweets = query::find_tweets(&**pool).await?;
    Ok(Json(tweets))
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
    command::Tweet::store(&**pool, &tweet).await?;
    Ok(Json(()))
}
