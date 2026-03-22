use actix_web::{get, post, web, HttpResponse, Scope};
use crate::service::user_service::UserService;
use crate::model::user::UserCreate;

#[get("/users")]
async fn list() -> HttpResponse {
    let svc = UserService::default();
    HttpResponse::Ok().json(svc.list_users())
}

#[post("/users")]
async fn create(payload: web::Json<UserCreate>) -> HttpResponse {
    let svc = UserService::default();
    let created = svc.create_user(payload.0);
    HttpResponse::Created().json(created)
}

pub fn scope() -> Scope {
    web::scope("/api").service(list).service(create)
}