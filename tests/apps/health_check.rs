use actix_web::{test, web, App};

use lib::apps;

#[actix_rt::test]
async fn health_check_ok() {
    let mut app =
        test::init_service(App::new().route("/health_check", web::get().to(apps::health_check)))
            .await;
    let request = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let response = test::call_service(&mut app, request).await;
    assert!(response.status().is_success());
}
