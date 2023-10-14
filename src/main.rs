mod database;
mod config;
mod logger;
mod common;

use std;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use crate::config::config::SETTING;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("server is running healthy")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = SETTING.server.port;
    println!("service is running in http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
