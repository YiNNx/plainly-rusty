mod config;
mod entities;
mod graphgql;
mod utilities;

use actix_web::{guard, web, web::Data, App, HttpServer};
use sea_orm::Database;

use config::global_config;
use graphgql::index::{graphql_playground, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database = Database::connect(&*global_config().database.url)
        .await
        .expect("failed to connect to the postgres");
    let schema = graphgql::schema::schema(database).expect("failed to load schema");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
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
    })
    .bind(&*global_config().application.address)?
    .run()
    .await
}
