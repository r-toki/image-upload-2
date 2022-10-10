mod blobs;
mod tweets;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    blobs::init(cfg);
    tweets::init(cfg);
}
