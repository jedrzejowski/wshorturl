mod routes;
mod database;
mod config;

use actix_web::{App, HttpServer};
use crate::config::{APP_ENV};
use crate::database::{Database};
use crate::database::yaml_database::YamlFileDatabase;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database: Database;

    println!("{:?}", APP_ENV.yaml_file);

    database = YamlFileDatabase::from_file(
        APP_ENV.yaml_file.as_ref().unwrap_or(&"database.yml".to_string())
    ).unwrap().into();

    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .service(routes::default::redirect)
    })
        .bind((
            APP_ENV.host.as_ref().unwrap_or(&"127.0.0.1".to_string()).clone(),
            APP_ENV.port.as_ref().unwrap_or(&8080).clone()
        ))?
        .run()
        .await
}