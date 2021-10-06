use actix_web::{get, web, App, HttpServer, Responder};
use std::io;

// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

async fn status() -> impl Responder { 
    "{\"status\": \"UP\"}"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Starting server at http://localhost:8080/");
    
    HttpServer::new(|| {
        App::new().route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}