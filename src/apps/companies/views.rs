use actix_web::{http::header, web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::apps::companies::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompany {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCompany {
    pub name: Option<String>,
}

pub async fn list() -> Result<HttpResponse, Error> {
    Ok(web::block(move || models::Company::all())
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create(
    json: web::Json<CreateCompany>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let item = models::CreateCompany {
        name: json.into_inner().name,
    };
    Ok(web::block(move || item.create())
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
    web::Path(id): web::Path<i32>,
    json: web::Json<UpdateCompany>,
) -> Result<HttpResponse, Error> {
    let item = web::block(move || models::Company::id(id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match item {
        Some(item) => Ok(web::block(move || item.update(&json.into_inner()))
            .await
            .map(|item| HttpResponse::Ok().json(item))
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn retrieve(web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || models::Company::id(id))
        .await
        .map(|item| match item {
            Some(v) => HttpResponse::Ok().json(v),
            None => HttpResponse::NotFound().finish(),
        })
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn destroy(web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = web::block(move || models::Company::id(id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match item {
        Some(item) => Ok(web::block(move || item.delete())
            .await
            .map(|_| HttpResponse::NoContent().finish())
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
