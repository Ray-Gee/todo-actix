mod models;
mod config;

use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTLS;

async fn status() -> impl Responder { 
    web::HttpResponse::Ok().json(Status {status: "ok".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTLS).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);
    
    HttpServer::new(move|| {
        App::new().data(pool.clone()).route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}