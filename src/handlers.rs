use crate::config;
use actix_web::{get, head, patch, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use slog::Logger;
use std::sync::Arc;
use storage::backend::BlobBackend;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub logger: Logger,
    pub config: config::Config,
    pub backend: Arc<dyn BlobBackend + Send + Sync>,
}

pub async fn v2() -> impl Responder {
    HttpResponse::Ok().body("{}")
}

#[get("/info")]
pub async fn backend_info(data: web::Data<AppState>) -> impl Responder {
    format!("backend type: {:?}", data.config.storage.backend_type)
}

/**
 * Path params
 */

#[derive(Deserialize)]
pub struct NameDigest {
    name: String,
    digest: String,
}

#[derive(Deserialize)]
pub struct NameReference {
    name: String,
    reference: String,
}

#[derive(Deserialize)]
pub struct NameUUID {
    name: String,
    uuid: String,
}

/**
 * query string params
 */

#[derive(Deserialize)]
pub struct QueryDigest {
    digest: String,
}

/**
 * handlers for pulling
 */

// TODO: check blob
#[head("/{name:.*}/blobs/{digest}")]
pub async fn check_blob(data: web::Data<AppState>, info: web::Path<NameDigest>) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.CHECK]";"name"=>&info.name,"digest"=>&info.digest,
    );
    HttpResponse::Ok().body(format!("name: {}, digest: {}", info.name, info.digest))
}

// TODO: download blob
#[get("/{name:.*}/blobs/{digest}")]
pub async fn download_blob(
    data: web::Data<AppState>,
    info: web::Path<NameDigest>,
) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.DOWNLOAD]";"name"=>&info.name,"digest"=>&info.digest,
    );
    HttpResponse::Ok().body(format!("name: {}, digest: {}", info.name, info.digest))
}

// TODO: check manifest
#[head("/{name:.*}/manifests/{reference}")]
pub async fn check_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
) -> impl Responder {
    info!(
        data.logger,
        "[MANIFEST.CHECK]";"name"=>&info.name,"reference"=>&info.reference,
    );
    HttpResponse::Ok()
}

// TODO: download manifest
#[get("/{name:.*}/manifests/{reference}")]
pub async fn download_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
) -> impl Responder {
    info!(
        data.logger,
        "[MANIFEST.DOWNLOAD]";"name"=>&info.name,"reference"=>&info.reference,
    );
    HttpResponse::Ok()
}

// TODO: upload manifest
#[put("/{name:.*}/manifests/{reference}")]
pub async fn upload_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
) -> impl Responder {
    info!(
        data.logger,
        "[MANIFEST.UPLOAD]";"name"=>&info.name,"reference"=>&info.reference,
    );
    HttpResponse::Ok()
}

/**
 * handlers for upload layer
 * **/

// TODO: init upload
#[post("/{name:.*}/blobs/uploads/")]
pub async fn init_upload(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    info!(data.logger, "[BLOB.INIT_UPLOAD]";"name"=>name.as_ref());
    HttpResponse::TemporaryRedirect()
        .header(
            "location",
            format!("/v2/{}/blobs/uploads/{}", name, Uuid::new_v4()),
        )
        .body("{}")
}

// TODO: get upload status
#[get("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn status_upload(data: web::Data<AppState>, info: web::Path<NameUUID>) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.STATUS_UPLOAD]";"name"=>&info.name, "uuid"=>&info.uuid,
    );
    HttpResponse::Ok()
}

// TODO: monolithic upload blob
#[put("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn monolithic_upload(
    data: web::Data<AppState>,
    info: web::Path<NameUUID>,
    query: web::Query<QueryDigest>,
) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.MONOLITHIC_UPLOAD]";"name"=>&info.name, "uuid"=>&info.uuid,"digest"=>&query.digest,
    );

    HttpResponse::Ok()
}

// TODO: checked upload blob
#[patch("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn chunked_upload(
    data: web::Data<AppState>,
    info: web::Path<NameUUID>,
) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.chunked_upload]";"name"=>&info.name, "uuid"=>&info.uuid,
    );

    HttpResponse::Ok()
}
