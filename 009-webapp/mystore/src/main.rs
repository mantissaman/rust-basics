pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

use actix_web::{HttpServer, App, web, get, HttpRequest, HttpResponse};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(
        web::resource("/").route(web::get().to(handlers::products::index)
    ))
    .bind("127.0.0.1:8080")
    .unwrap()
    .run().await
}
