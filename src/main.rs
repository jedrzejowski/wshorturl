mod routes;
mod database;
mod config;

use actix_web::{App, HttpServer};
use crate::config::{APP_ENV, AppEnv};
use crate::database::{Database};
use crate::database::yaml_database::YamlFileDatabase;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database: Database;

    database = YamlFileDatabase::from_file(
        APP_ENV.yaml_file.unwrap_or("database.yml".to_string())
    ).unwrap().into();

    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .service(routes::default::redirect)
    })
        .bind((
            APP_ENV.host.unwrap_or("127.0.0.1".to_string()),
            APP_ENV.port.unwrap_or(8080)
        ))?
        .run()
        .await
}