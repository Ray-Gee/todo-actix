use actix_web::{web, web::ServiceConfig, HttpResponse};

pub fn app_config(config: &mut ServiceConfig) {
    let health_recource = web::resource("/")
        .route(web::get().to(health));

    config.service(health_recource);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}