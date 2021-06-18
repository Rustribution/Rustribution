// TODO: tags api handlers
use actix_web::{
  get,
  // patch, post, put,
  web,
  HttpResponse,
  Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryPaginated {
  pub _n: Option<u64>,
  pub _last: Option<u64>,
}

#[get("/{name:.*}/tags/list")]
pub async fn tags_list(_paginated: web::Query<QueryPaginated>) -> impl Responder {
  HttpResponse::Ok().body("{}")
}
