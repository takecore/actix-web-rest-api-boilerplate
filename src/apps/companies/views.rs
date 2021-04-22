use actix_web::{http::header, web, Error, HttpRequest, HttpResponse};

use crate::apps::companies::models;

pub async fn list() -> Result<HttpResponse, Error> {
    Ok(web::block(move || models::Company::all())
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create(
    web::Json(json): web::Json<models::CreateCompany>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let create = models::CreateCompany { name: json.name };
    Ok(web::block(move || models::Company::create(&create))
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
    web::Json(json): web::Json<models::UpdateCompany>,
) -> Result<HttpResponse, Error> {
    let item = web::block(move || models::Company::id(id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(match item {
        Some(item) => web::block(move || item.update(&json))
            .await
            .map(|item| HttpResponse::Ok().json(item))
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?,
        None => HttpResponse::NotFound().finish(),
    })
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

    Ok(match item {
        Some(item) => web::block(move || item.delete())
            .await
            .map(|_| HttpResponse::NoContent().finish())
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?,
        None => HttpResponse::NotFound().finish(),
    })
}
