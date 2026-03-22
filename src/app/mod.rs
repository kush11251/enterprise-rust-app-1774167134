use actix_web::{web, App, HttpServer};
use crate::controllers::user_controller;

pub async fn run_app() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(user_controller::scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}