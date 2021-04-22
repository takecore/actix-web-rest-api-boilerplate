use actix_web::{web, HttpResponse, Responder};

pub mod companies;
pub mod users;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello actix-web2-rest-api!"))
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// confirm bloking behavior. can delete lines.
pub async fn blocking() -> impl Responder {
    let duration = std::time::Duration::from_secs(10);
    std::thread::sleep(duration);
    HttpResponse::Ok().body(format!(
        "run resource. blocked {} sec.",
        duration.as_secs().to_string()
    ))
}

// confirm non bloking behavior. can delete lines.
pub async fn nonblocking() -> impl Responder {
    let duration = std::time::Duration::from_secs(10);
    let items = web::block(move || {
        std::thread::sleep(duration);
        crate::apps::companies::models::Company::all()
    })
    .await
    .unwrap();
    HttpResponse::Ok().body(format!(
        "found {} items. blocked {} sec.",
        items.len(),
        duration.as_secs().to_string()
    ))
}
