use actix_web::{http::header, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::apps::companies::models::Company;
use crate::apps::users::models;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

pub async fn list() -> Result<HttpResponse, AppError> {
    let items = web::block(move || models::User::all()).await?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn create(
    web::Path(company_id): web::Path<i32>,
    web::Json(json): web::Json<CreateUser>,
    request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let (company, created) = web::block(move || -> Result<(Company, models::User), AppError> {
        let company = Company::id(company_id)?;
        let user = models::CreateUser {
            company_id: company.id,
            name: json.name,
        };
        Ok((company, models::User::create(&user)?))
    })
    .await?;

    let url = request.url_for(
        "user-detail",
        &[company.id.to_string(), created.id.to_string()],
    );
    Ok(HttpResponse::Created()
        .header(header::LOCATION, url.ok().unwrap().as_str())
        .json(created))
}

pub async fn update(
    web::Path((company_id, user_id)): web::Path<(i32, i32)>,
    web::Json(json): web::Json<models::UpdateUser>,
) -> Result<HttpResponse, AppError> {
    let updated = web::block(move || {
        let item = models::User::id_with_company_id(user_id, company_id)?;
        item.update(&json)
    })
    .await?;

    Ok(HttpResponse::Ok().json(updated))
}

pub async fn retrieve(
    web::Path((company_id, user_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let item = web::block(move || models::User::id_with_company_id(user_id, company_id)).await?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn destroy(
    web::Path((company_id, user_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let _ = web::block(move || {
        let item = models::User::id_with_company_id(user_id, company_id)?;
        item.delete()
    })
    .await?;

    Ok(HttpResponse::NoContent().finish())
}
