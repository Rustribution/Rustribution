use crate::{AppState, QueryDigest, QueryMount};
use actix_web::{post, web, HttpResponse, Responder};
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
    HttpResponse::Created()
        .header("Range", "0-0")
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
