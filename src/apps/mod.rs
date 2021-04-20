use std::sync::Mutex;

use crate::config;
use actix_web::{web, HttpResponse, Responder};

pub mod companies;
pub mod users;

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn hello(data: web::Data<config::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn countup(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    HttpResponse::Ok().body(format!("{}", counter))
}
