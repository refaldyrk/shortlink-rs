// service.rs
use actix_web::{get, HttpResponse, post, Responder, web};
use moka::sync::Cache;
use std::sync::Arc;
use actix_web::web::Redirect;
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
            message: "already exists".to_string()
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

#[get("/{id}")]
pub async fn get_links(path: web::Path<String>, c: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let cache = &c.cache;

    if let Some(long) = cache.get(&id) {
        //Redirect To Long
        Redirect::to(long.clone()).temporary()
    } else {
        //Redirect To Web Rust If Not Found
        Redirect::to("https://www.rust-lang.org/").temporary()
    }
}