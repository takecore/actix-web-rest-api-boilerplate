use actix_web::{web, Error, HttpResponse, Responder};
// use futures::future::{ready, Ready};

use crate::apps::user::models::User;
use crate::db::DbPool;

pub async fn list(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Ok(web::block(move || User::all(&conn))
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("user create")
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("user update")
}

// pub async fn retrieve() -> impl Responder {
//     User { name: "takecore" }
// }

pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("user destroy")
}
