use std::io;

use actix_web::{
    body::BoxBody, error::HttpError, get, http::header::ContentType, post, web, App, Either,
    HttpRequest, HttpResponse, Responder, ResponseError,
};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hi!")
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct Info2 {
    username: String,
}

/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info2>) -> Result<String, HttpError> {
    Ok(format!("Welcome {}!", info.username))
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn index2() -> impl Responder {
    MyObj { name: "user" }
}

use actix_web::{error, http::StatusCode};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/")]
async fn index3() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}

#[derive(Debug)]
struct MyError2 {
    name: &'static str,
}

#[get("/")]
async fn index4() -> actix_web::Result<String> {
    let result = Err(MyError2 { name: "test error" });

    result.map_err(|err| error::ErrorBadRequest(err.name))
}

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/err")]
async fn index5() -> Result<&'static str, UserError> {
    //do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}