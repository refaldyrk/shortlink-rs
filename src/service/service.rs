// service.rs
use actix_web::{HttpResponse, post, Responder, web};
use moka::sync::Cache;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::model;

pub struct AppState {
    pub cache: Arc<Cache<String, String>>
}

#[post("/")]
pub async fn add_links(data: web::Data<AppState>, info: web::Json<model::links::Link>) -> impl Responder {
    #[derive(Serialize, Deserialize)]
    struct Response {
        message: String
    }

    let cache = &data.cache;
    let key = info.short.to_string();

    if let Some(_hello) = cache.get(&key) {
        let error_response = Response {
            message: "Key already exists".to_string()
        };

        HttpResponse::Conflict().json(error_response)
    } else {
        cache.insert(key.clone(), info.long.to_string());

        let message_response = Response {
            message: "Insert successful".to_string()
        };

        HttpResponse::Created().json(message_response)
    }
}
