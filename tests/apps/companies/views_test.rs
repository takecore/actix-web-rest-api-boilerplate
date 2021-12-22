use actix_web::{test, App};
use diesel_cli::database::reset_database;

use lib::apps::companies::models::Company;
use lib::server::routes;

#[actix_rt::test]
async fn ccc() {
    // let mut app = test::init_service(App::new().configure(routes)).await;
    // let request = test::TestRequest::with_header("content-type", "application/json")
    //     .uri("/companies")
    //     .to_request();
    // let response: Vec<Company> = test::read_response_json(&mut app, request).await;
    // assert_eq!(response.len(), 0);
}

#[actix_rt::test]
async fn ddd() {
    assert!(true);
}
