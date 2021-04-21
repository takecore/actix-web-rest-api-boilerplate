use actix_web::{HttpResponse, Responder};

pub mod companies;
pub mod users;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello actix-web2-rest-api!"))
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
