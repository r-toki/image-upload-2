mod controllers;
mod lib;
mod models;

use crate::lib::{config::CONFIG, cors::cors};

use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let pool = PgPool::connect(&CONFIG.database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(cors())
            .service(index)
            .configure(controllers::init)
    })
    .bind(format!("{}:{}", CONFIG.host, CONFIG.port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    format!("HELLO WORLD")
}
