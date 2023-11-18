mod config;
mod entities;
mod query;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    dynamic::*,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use config::Config;
use sea_orm::Database;

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            &Config().graphql.endpoint,
        ))))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();
    let database = Database::connect(&*Config().database.url)
        .await
        .expect("Fail to initialize database connection");
    let schema = query::root::schema(database).unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource(&*Config().graphql.endpoint)
                    .guard(guard::Post())
                    .to(index),
            )
            .service(
                web::resource(&*Config().graphql.endpoint)
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(&*Config().application.address)?
    .run()
    .await
}
