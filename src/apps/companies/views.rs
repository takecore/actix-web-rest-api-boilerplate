use actix_web::{http::header, web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::apps::companies::models;
use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompany {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCompany {
    pub name: Option<String>,
}

// TODO get connection from pool on middleware
pub async fn list(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Ok(web::block(move || models::Company::all(&conn))
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create(
    pool: web::Data<DbPool>,
    json: web::Json<CreateCompany>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let item = models::CreateCompany {
        name: json.into_inner().name,
    };
    Ok(web::block(move || item.create(&conn))
        .await
        .map(|item| {
            let url = request.url_for("company-detail", &[item.id.to_string()]);
            HttpResponse::Created()
                .header(header::LOCATION, url.ok().unwrap().as_str())
                .json(item)
        })
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?)
}

pub async fn update(
    pool: web::Data<DbPool>,
    web::Path(id): web::Path<i32>,
    json: web::Json<UpdateCompany>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let item = web::block(move || models::Company::id(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match item {
        Some(item) => {
            let conn = pool.get().expect("couldn't get db connection from pool");
            Ok(web::block(move || item.update(&conn, &json.into_inner()))
                .await
                .map(|item| HttpResponse::Ok().json(item))
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?)
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn retrieve(
    pool: web::Data<DbPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Ok(web::block(move || models::Company::id(&conn, id))
        .await
        .map(|item| match item {
            Some(v) => HttpResponse::Ok().json(v),
            None => HttpResponse::NotFound().finish(),
        })
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn destroy(
    pool: web::Data<DbPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let item = web::block(move || models::Company::id(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match item {
        Some(item) => {
            let conn = pool.get().expect("couldn't get db connection from pool");
            Ok(web::block(move || item.delete(&conn))
                .await
                .map(|_| HttpResponse::NoContent().finish())
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?)
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
