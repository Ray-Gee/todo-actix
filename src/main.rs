mod models;
mod config;
mod handlers;
mod db;

use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);
    
    HttpServer::new(move|| {
        App::new().data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}