mod handlers;
mod middlewares;
mod routers;
mod utilities;

use crate::utilities::config::config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use middlewares::hello::SayHi;
use routers::routers::{router_post, router_user};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();

//     HttpServer::new(|| {
//         App::new()
//             .wrap(SayHi)
//             .wrap(Logger::default())
//             .service(router_user())
//             .service(router_post())
//     })
//     .bind((
//         config().application.host.as_str(),
//         config().application.port,
//     ))?
//     .run()
//     .await
// }

// main.rs

use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend() {
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        _ => db,
    };

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
