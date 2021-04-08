use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: &'static str,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

pub async fn list() -> impl Responder {
    HttpResponse::Ok().body("user list")
}

pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("user create")
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("user update")
}

pub async fn retrieve() -> impl Responder {
    User { name: "takecore" }
}

pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("user destroy")
}
