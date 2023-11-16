use actix_web::{web, App, HttpResponse, Resource, Scope};

use crate::handlers::user;

const SCOPE_USER: &str = "user";
const SCOPE_POST: &str = "post";
const SCOPE_COMMENT: &str = "comment";

pub fn condig_post(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub fn router_user() -> Scope {
    web::scope(SCOPE_USER).route("/index.html", web::get().to(user::hello))
}

pub fn router_post() -> Scope {
    web::scope(SCOPE_POST)
        .route("/index.html", web::get().to(user::hello))
        .configure(condig_post)
}
