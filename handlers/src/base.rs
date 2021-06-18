use actix_web::{http::header, HttpResponse, Responder};

pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body("{}")
}
