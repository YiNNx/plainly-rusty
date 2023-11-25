mod config;
mod entities;
mod services;
mod utilities;

use actix_cors::Cors;
use actix_web::{guard, web, web::Data, App, HttpServer};
use config::global_config;
use sea_orm::Database;
use services::{
    graphql::{graphql_playground, index},
    rss::rss,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database = Database::connect(&*global_config().database.url)
        .await
        .expect("failed to connect to the database");
    let schema = services::schema::schema(database.clone()).expect("failed to load schema");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(database.clone()))
            .service(
                web::resource(&*global_config().graphql.endpoint)
                    .guard(guard::Post())
                    .to(index),
            )
            .service(
                web::resource(&*global_config().graphql.endpoint)
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
            .service(web::resource("/rss.xml").guard(guard::Get()).to(rss))
    })
    .bind(&*global_config().application.address)?
    .run()
    .await
}
