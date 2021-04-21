use actix_web::{web, App, HttpServer};

use crate::apps;
use crate::db;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/companies/{company_id}/users/{user_id}")
            .name("user-detail")
            .route(web::get().to(apps::users::views::retrieve))
            .route(web::post().to(apps::users::views::update))
            .route(web::delete().to(apps::users::views::destroy)),
    )
    .service(
        web::resource("/companies/{company_id}/users")
            .name("user-list")
            .route(web::get().to(apps::users::views::list))
            .route(web::post().to(apps::users::views::create)),
    )
    .service(
        web::resource("/companies/{id}")
            .name("company-detail")
            .route(web::get().to(apps::companies::views::retrieve))
            .route(web::post().to(apps::companies::views::update))
            .route(web::delete().to(apps::companies::views::destroy)),
    )
    .service(
        web::resource("/companies")
            .name("company-list")
            .route(web::get().to(apps::companies::views::list))
            .route(web::post().to(apps::companies::views::create)),
    )
    .route("/echo", web::get().to(apps::echo))
    .route("/", web::get().to(apps::hello));
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    db::init();

    HttpServer::new(move || App::new().configure(routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
