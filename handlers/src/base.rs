use actix_web::{HttpResponse, Responder};

pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body("{}")
}
