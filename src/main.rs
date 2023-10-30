// main.rs
use std::sync::Arc;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use moka::sync::Cache;
use serde::Serialize;
mod service;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Cache
    let cache: Arc<Cache<String, String>> = Arc::new(Cache::new(1000));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service::service::AppState {
                cache: Arc::clone(&cache)
            }))
            .service(service::service::add_links)
            .service(service::service::get_links)
            .route("/", web::get().to(health_check))
    })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}

async fn health_check() -> impl Responder {
    #[derive(Serialize)]
    struct Response {
        message: String
    }

    let message_response = Response {
        message: "Everything Ok".to_string()
    };

    HttpResponse::Ok().json(message_response)
}
