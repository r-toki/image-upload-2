mod lib;
pub mod storages;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    storages::init(cfg);
}
