use actix_web::{http::header, web, Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::apps::companies::models::Company;
use crate::apps::users::models;
use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

// TODO get connection from pool on middleware
pub async fn list(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Ok(web::block(move || models::User::all(&conn))
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create(
    pool: web::Data<DbPool>,
    web::Path(company_id): web::Path<i32>,
    web::Json(json): web::Json<CreateUser>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let company = web::block(move || Company::id(&conn, company_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match company {
        Some(company) => {
            let conn = pool.get().expect("couldn't get db connection from pool");
            let user = models::CreateUser {
                company_id: company.id,
                name: json.name,
            };
            Ok(web::block(move || user.create(&conn))
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
                })?)
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("user update")
}

pub async fn retrieve(
    pool: web::Data<DbPool>,
    web::Path((company_id, user_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Ok(
        web::block(move || models::User::id_with_company_id(&conn, user_id, company_id))
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
