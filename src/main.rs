mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::product_api::{create_product, delete_product, get_all_products, get_product, update_product};
use repository::mongodb_repo::MongoRepo;
use actix_web::{middleware::Logger};
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_product)
            .service(get_product)
            .service(update_product)
            .service(delete_product)
            .service(get_all_products)
            .wrap(Logger::new("%a %r %s %b %{Referer}i %T"))
    })
    .bind((host.as_str(), port))?;

    info!("Server running at http://{}:{}", host, port);
    server.run().await
}
