use actix_web::{http::header, web, HttpRequest, HttpResponse};

use crate::apps::companies::models;
use crate::error::AppError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub name: Option<String>,
}

pub async fn list(web::Query(query): web::Query<SearchQuery>) -> Result<HttpResponse, AppError> {
    let items = web::block(move || models::Company::search(query)).await?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn create(
    web::Json(json): web::Json<models::CreateCompany>,
    request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let create = models::CreateCompany { name: json.name };
    let item = web::block(move || models::Company::create(&create)).await?;

    let url = request.url_for("company-detail", &[item.id.to_string()]);
    Ok(HttpResponse::Created()
        .header(header::LOCATION, url.ok().unwrap().as_str())
        .json(item))
}

pub async fn update(
    web::Path(id): web::Path<i32>,
    web::Json(json): web::Json<models::UpdateCompany>,
) -> Result<HttpResponse, AppError> {
    let updated = web::block(move || {
        let item = models::Company::id(id)?;
        item.update(&json)
    })
    .await?;

    Ok(HttpResponse::Ok().json(updated))
}

pub async fn retrieve(web::Path(id): web::Path<i32>) -> Result<HttpResponse, AppError> {
    let item = web::block(move || models::Company::id(id)).await?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn destroy(web::Path(id): web::Path<i32>) -> Result<HttpResponse, AppError> {
    let _ = web::block(move || {
        let item = models::Company::id(id)?;
        item.delete()
    })
    .await?;

    Ok(HttpResponse::NoContent().finish())
}
