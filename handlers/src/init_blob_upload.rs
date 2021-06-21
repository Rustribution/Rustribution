use crate::hmac::{BlobUploadState, UploadStater};
use crate::{AppState, QueryDigest, QueryMount, DATATIME_FMT};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::prelude::NaiveDateTime;
use uuid::Uuid;

#[post("/{name:.*}/blobs/uploads/")]
pub async fn init_upload(
    data: web::Data<AppState>,
    name: web::Path<String>,
    query: web::Query<QueryDigest>,
    mount: web::Query<QueryMount>,
) -> impl Responder {
    let digest = query.digest.clone().unwrap_or(String::from(""));
    let mount_digest = mount.mount.clone().unwrap_or(String::from(""));
    let mount_from = mount.from.clone().unwrap_or(String::from(""));
    let conditions: (bool, bool, bool) = (
        digest.is_empty(),
        mount_digest.is_empty(),
        mount_from.is_empty(),
    );
    match conditions {
        (false, true, true) => monolithic_upload(data, name.clone(), digest),
        (true, true, true) => resumable_upload(data, name.clone()),
        (true, false, false) => mount_blob(data, mount_from, mount_digest),
        _ => bad_init_upload(),
    }
}

fn monolithic_upload(data: web::Data<AppState>, name: String, digest: String) -> HttpResponse {
    // TODO: get Content-Length
    let id = Uuid::new_v4().to_string();
    let location = format!("/v2/{}/blobs/uploads/{}", name, id);

    info!(
        data.logger,
        "[BLOB.INIT.MONOLITHIC_UPLOAD]";
        "name"=>&name.clone(),
        "digest"=>digest.clone(),
        "session"=>id.clone(),
    );

    debug!(
        data.logger,"[BLOB.INIT.MONOLITHIC_UPLOAD]";
        "location"=>location.clone(),
        "name"=>&name.clone(), "digest"=>digest.clone(),
    );
    HttpResponse::Created()
        .header("Location", location) // TODO
        .header("Docker-Upload-UUID", id)
        .body("")
}

fn resumable_upload(data: web::Data<AppState>, name: String) -> HttpResponse {
    info!(
        data.logger,
        "[BLOB.INIT.RESUMABLE_UPLOAD]";"name"=>&name.clone(),
    );

    let id = Uuid::new_v4().to_string();
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
        .header("Range", "0-0")
        .header(
            "Location",
            format!("/v2/{}/blobs/uploads/{}?_state={}", name, id, statestr),
        )
        .header("Docker-Upload-UUID", id)
        .body("")
}

fn mount_blob(data: web::Data<AppState>, from: String, digest: String) -> HttpResponse {
    info!(data.logger, "[BLOB.INIT.MOUNT]";
    "from"=>from,
    "digest"=>digest
    );
    HttpResponse::Ok().body("")
}

fn bad_init_upload() -> HttpResponse {
    HttpResponse::BadRequest().body("")
}
