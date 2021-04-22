use actix_web::{http::header, web, Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::apps::companies::models::Company;
use crate::apps::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

pub async fn list() -> Result<HttpResponse, Error> {
    Ok(web::block(move || models::User::all())
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create(
    web::Path(company_id): web::Path<i32>,
    web::Json(json): web::Json<models::CreateUser>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let company = web::block(move || Company::id(company_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(match company {
        Some(company) => {
            let user = models::CreateUser {
                company_id: company.id,
                name: json.name,
            };
            web::block(move || user.create())
                .await
                .map(|user| {
                    let url = request.url_for(
                        "user-detail",
                        &[company.id.to_string(), user.id.to_string()],
                    );
                    HttpResponse::Created()
                        .header(header::LOCATION, url.ok().unwrap().as_str())
                        .json(user)
                })
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?
        }
        None => HttpResponse::NotFound().finish(),
    })
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("user update")
}

pub async fn retrieve(
    web::Path((company_id, user_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || models::User::id_with_company_id(user_id, company_id))
            .await
            .map(|item| match item {
                Some(v) => HttpResponse::Ok().json(v),
                None => HttpResponse::NotFound().finish(),
            })
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("user destroy")
}
