use crate::models::Status;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder};

pub async fn status() -> impl Responder { 
    web::HttpResponse::Ok().json(Status {status: "ok".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {}