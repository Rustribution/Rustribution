use crate::hmac::UploadStater;
use crate::utils;
use crate::DOCKER_CONTENT_DIGEST;
use crate::{build_blob_path, build_blob_temp_upload_path};
use crate::{AppState, NameUUID, QueryDigest, QueryState, DOCKER_UPLOAD_UUID};
use actix_web::{delete, get, http, patch, put, web, HttpRequest, HttpResponse};
use bytes::Bytes;

// TODO
/// GET Blob Upload
/// Retrieve status of upload identified by uuid. The primary purpose of this endpoint is to resolve the current status of a resumable upload.
/// The Content-Length header must be zero and the body must be empty.
#[get("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn status_upload(data: web::Data<AppState>, info: web::Path<NameUUID>) -> HttpResponse {
    info!(
        data.logger,"[BLOBUPLOAD.STATUS]";
        "name"=>info.clone().name,
        "uuid"=>info.clone().uuid,
    );

    // TODO: retrieve offset
    let offset: u64 = 0;
    HttpResponse::Ok()
        .header(DOCKER_UPLOAD_UUID, info.clone().uuid)
        .header(http::header::RANGE, format!("0-{}", offset))
        .header(http::header::CONTENT_LENGTH, "0")
        .finish()
}

/// PATCH Blob Upload
/// Upload a chunk of data for the specified upload.
/// Stream upload
#[patch("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn stream_upload(
    data: web::Data<AppState>,
    info: web::Path<NameUUID>,
    query_state: web::Query<QueryState>,
    body: Bytes,
    req: HttpRequest,
) -> HttpResponse {
    // let range = req
    //   .headers()
    //   .get("Content-Range")
    //   .unwrap()
    //   .to_str()
    //   .unwrap_or("");

    let size = body.len();
    let length = utils::get_content_length(req.headers());

    info!(
        data.logger,"[BLOBUPLOAD.CHUNK]";
        "name"=>info.clone().name,
        "uuid"=>info.clone().uuid,
        "body.size"=>&size,
        "content.length"=>&length,
    );

    if size < length {
        HttpResponse::BadRequest().body("client disconnected")
    } else {
        let mut state = UploadStater::new(data.http_secret.clone())
            .unpack(query_state._state.clone())
            .unwrap();
        state.offset += size as u64;

        let statestr = UploadStater::new(data.http_secret.clone())
            .pack(state.clone())
            .unwrap();

        let temp_path = build_blob_temp_upload_path(info.name.clone(), info.uuid.clone());
        let backend = &data.backend;
        backend.put_content(temp_path.clone(), body);
        let temp_size = backend.stat(temp_path.clone()).unwrap_or(0);
        debug!(
          data.logger,
          "upload chunk info";
          "uuid"=>&info.uuid,
          "name"=>&info.name,
          "temp.path"=>temp_path,
          "temp.size"=>temp_size,
          "body.size"=>size,
        );
        HttpResponse::Accepted()
            .header(
                http::header::LOCATION,
                format!(
                    "/v2/{}/blobs/uploads/{}?_state={}",
                    info.name.clone(),
                    info.uuid.clone(),
                    statestr
                ),
            )
            .header(http::header::RANGE, format!("bytes=0-{}", state.offset - 1))
            .header(http::header::CONTENT_LENGTH, "0")
            .header(crate::DOCKER_UPLOAD_UUID, info.uuid.clone())
            .finish()
    }
}

/// PUT Blob Upload
/// Complete the upload specified by uuid, optionally appending the body as the final chunk.
#[put("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn finish_upload(
    data: web::Data<AppState>,
    info: web::Path<NameUUID>,
    query: web::Query<QueryDigest>,
    req: HttpRequest,
    body: Bytes,
) -> HttpResponse {
    let body_size = body.len();
    let length = utils::get_content_length(req.headers());
    let src_path = build_blob_temp_upload_path(info.clone().name, info.clone().uuid);
    if body_size > 0 {
        // TODO: append to temp file
    }

    let digest = query.clone().digest.unwrap_or(String::new());
    let dst_path = build_blob_path(digest.clone());

    info!(
        data.logger,"[BLOBUPLOAD.FINISH]";
        "name"=>info.clone().name,
        "uuid"=>info.clone().uuid,
        "digest"=>&digest,
        "content.length"=>&length,
    );
    match data.backend.mov(src_path, dst_path.clone()) {
        Ok(_) => HttpResponse::Created()
            .header(
                http::header::LOCATION,
                format!("/v2/{}/blobs/{}", info.name, digest),
            )
            .header(http::header::CONTENT_RANGE, format!("0-{}", length))
            .header(DOCKER_CONTENT_DIGEST, digest)
            .finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// DELETE Blob Upload
/// Cancel outstanding upload processes, releasing associated resources. If this is not called, the unfinished uploads will eventually timeout.
#[delete("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn delete_upload(
    data: web::Data<AppState>,
    info: web::Path<NameUUID>,
    query: web::Query<QueryDigest>,
) -> HttpResponse {
    let name = info.clone().name;
    let uuid = info.clone().uuid;
    let digest = query.clone().digest.unwrap();
    info!(
        data.logger,"[BLOBUPLOAD.DELETE]";
        "name"=>&name,
        "uuid"=>&uuid,
        "digest"=>&digest,
    );
    let temp_path = build_blob_temp_upload_path(name.clone(), uuid);
    match data.backend.delete(temp_path.clone()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            error!(data.logger,
              "cancel outstanding upload processes failed";
              "name"=> name,
              "path"=> temp_path,
              "error"=>err.to_string(),
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
