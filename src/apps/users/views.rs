use actix_web::{web, Error, HttpResponse, Responder};

use crate::apps::users::models::User;
use crate::db::DbPool;

// TODO get connection from pool on middleware
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

pub async fn retrieve(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Ok(web::block(move || User::id(&conn, id.into_inner()))
        .await
        .map(|item| match item {
            Some(v) => HttpResponse::Ok().json(v),
            None => HttpResponse::NotFound().finish(),
        })
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("user destroy")
}
