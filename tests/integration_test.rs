use actix_web::test;
use enterprise_rust::app::run_app;

#[actix_web::test]
async fn smoke_test() {
    let srv = test::start(|| run_app().await.unwrap());
    let response = srv.get("/api/users").send().await.unwrap();
    assert!(response.status().is_success());
}