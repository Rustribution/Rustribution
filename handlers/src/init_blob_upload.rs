use crate::build_blob_path;
use crate::hmac::{BlobUploadState, UploadStater};
use crate::utils;
use crate::{AppState, QueryDigest, QueryMount, DATATIME_FMT};
use crate::{DOCKER_CONTENT_DIGEST, DOCKER_UPLOAD_UUID};
use actix_web::{http, post, web, HttpRequest, HttpResponse, Responder};
use bytes::Bytes;
use chrono::prelude::NaiveDateTime;
use std::io::ErrorKind;
use uuid::Uuid;

#[post("/{name:.*}/blobs/uploads/")]
pub async fn init_upload(
    data: web::Data<AppState>,
    name: web::Path<String>,
    query: web::Query<QueryDigest>,
    mount: web::Query<QueryMount>,
    req: HttpRequest,
    body: Bytes,
) -> impl Responder {
    let digest = query.clone().digest.unwrap_or(String::from(""));
    let mount_digest = mount.mount.clone().unwrap_or(String::from(""));
    let mount_from = mount.from.clone().unwrap_or(String::from(""));
    let conditions: (bool, bool, bool) = (
        digest.is_empty(),
        mount_digest.is_empty(),
        mount_from.is_empty(),
    );
    info!(
        data.logger,"[BLOB.INIT.UPLOAD]";
        "conditions"=>format!("{:?}",conditions),
        "header"=>format!("{:?}",req.headers()),
    );
    match conditions {
        (false, true, true) => monolithic_upload(data, &name, &digest, req, body),
        (true, true, true) => resumable_upload(data, &name),
        (true, false, false) => mount_blob(data, &name, &mount_from, &mount_digest),
        _ => bad_init_upload(),
    }
}

fn monolithic_upload(
    data: web::Data<AppState>,
    name: &String,
    digest: &String,
    req: HttpRequest,
    body: Bytes,
) -> HttpResponse {
    let body_size = body.len();
    let length = utils::get_content_length(req.headers());

    let id = Uuid::new_v4().to_string();
    let location = format!("/v2/{}/blobs/uploads/{}", name, id);
    info!(
        data.logger,
        "[BLOB.INIT.MONOLITHIC_UPLOAD]";
        "name"=>&name,
        "digest"=>&digest,
        "session"=>&id,
        "location"=>&location,
    );
    if body_size != length {
        error!(
            data.logger,
            "body size not equal content-length header";
            "body.size"=>body_size,
            "content.length"=>length,
        );
        return HttpResponse::BadRequest().body("");
    }

    let path = build_blob_path(digest.clone());
    data.backend.put_content(path, body);
    HttpResponse::Created()
        .header(http::header::LOCATION, location) // TODO
        .header("Docker-Upload-UUID", id)
        .body("")
}

fn resumable_upload(data: web::Data<AppState>, name: &String) -> HttpResponse {
    let id = Uuid::new_v4().to_string();

    info!(
        data.logger,
        "[BLOB.INIT.RESUMABLE_UPLOAD]";
        "name"=>name,
        "id"=>&id,
    );

    let state = BlobUploadState {
        name: name.clone(),
        offset: 0,
        uuid: id.clone(),
        started_at: NaiveDateTime::parse_from_str("2021-06-19T06:36:04.97859", DATATIME_FMT)
            .unwrap(),
    };
    let statestr = UploadStater::new(data.http_secret.clone())
        .pack(state)
        .unwrap();
    HttpResponse::Accepted()
        .header(
            "Location",
            format!("/v2/{}/blobs/uploads/{}?_state={}", name, &id, statestr),
        )
        .header(DOCKER_UPLOAD_UUID, id)
        .finish()
}

fn mount_blob(
    data: web::Data<AppState>,
    name: &String,
    from: &String,
    digest: &String,
) -> HttpResponse {
    info!(data.logger, "[BLOB.INIT.MOUNT]";
    "name"=>name,
    "from"=>from,
    "digest"=>digest
    );

    let path = build_blob_path(digest.clone());
    match data.backend.stat(path) {
        Ok(_) => {
            // TODO: success
            HttpResponse::Created()
                .header("Location", format!("/v2/{}/blobs/{}", name, digest))
                .header(DOCKER_CONTENT_DIGEST, digest.clone())
                .finish()
        }
        Err(e) => match e.kind() {
            // TODO: If a mount fails due to invalid repository or digest arguments, the registry will fall back to the standard upload behavior and return a 202 Accepted with the upload URL in the Location header
            ErrorKind::NotFound => {
                // TODO:
                return resumable_upload(data, name);
            }
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

fn bad_init_upload() -> HttpResponse {
    HttpResponse::BadRequest().finish()
}
