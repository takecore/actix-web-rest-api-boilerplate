use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::apps::companies::models::Company;
use crate::apps::users::models;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

pub async fn list() -> Result<HttpResponse, AppError> {
    let items = web::block(move || models::User::all())
        .await
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn create(
    web::Path(company_id): web::Path<i32>,
    web::Json(json): web::Json<models::CreateUser>,
    request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // TODO describe get company and create user on 1 web::block
    let company = web::block(move || Company::id(company_id))
        .await
        .map_err(|e| AppError::from(e))?;

    let user = models::CreateUser {
        company_id: company.id,
        name: json.name,
    };
    let created = web::block(move || models::User::create(&user))
        .await
        .map_err(|e| AppError::from(e))?;

    let url = request.url_for(
        "user-detail",
        &[company.id.to_string(), created.id.to_string()],
    );
    Ok(HttpResponse::Created()
        .header(header::LOCATION, url.ok().unwrap().as_str())
        .json(created))
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("user update")
}

pub async fn retrieve(
    web::Path((company_id, user_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let item = web::block(move || models::User::id_with_company_id(user_id, company_id))
        .await
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("user destroy")
}
