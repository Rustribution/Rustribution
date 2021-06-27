use crate::hmac::UploadStater;
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
    .header("Range", format!("0-{}", offset))
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

  let length = req
    .headers()
    .get(http::header::CONTENT_LENGTH)
    .unwrap_or(&http::HeaderValue::from_str("0").unwrap())
    .to_str()
    .unwrap_or("")
    .parse::<usize>()
    .unwrap_or(0);

  let size = body.len();

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
    state.offset = size as u64;

    let statestr = UploadStater::new(data.http_secret.clone())
      .pack(state)
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
        "Location",
        format!(
          "/v2/{}/blobs/uploads/{}?_state={}",
          info.name.clone(),
          info.uuid.clone(),
          statestr
        ),
      )
      .header("Range", format!("0-{}", size - 1))
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
) -> HttpResponse {
  let length = req
    .headers()
    .get(http::header::CONTENT_LENGTH)
    .unwrap_or(&http::HeaderValue::from_str("0").unwrap())
    .to_str()
    .unwrap_or("")
    .parse::<usize>()
    .unwrap_or(0);

  let digest = query.clone().digest.unwrap_or(String::new());

  info!(
      data.logger,"[BLOBUPLOAD.FINISH]";
      "name"=>info.clone().name,
      "uuid"=>info.clone().uuid,
      "digest"=>&digest,
      "content.length"=>&length,
  );
  let src_path = build_blob_temp_upload_path(info.clone().name, info.clone().uuid);
  let dst_path = build_blob_path(digest.clone());
  match data.backend.mov(src_path, dst_path.clone()) {
    Ok(_) => HttpResponse::NoContent()
      .header("Location", format!("/v2/{}/blobs/{}", info.name, digest))
      .header("Content-Range", format!("0-{}", length))
      .header(DOCKER_CONTENT_DIGEST, digest)
      .finish(),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

// TODO
/// DELETE Blob Upload
/// Cancel outstanding upload processes, releasing associated resources. If this is not called, the unfinished uploads will eventually timeout.
#[delete("/{name:.*}/blobs/uploads/{uuid}")]
pub async fn delete_upload(
  data: web::Data<AppState>,
  info: web::Path<NameUUID>,
  query: web::Query<QueryDigest>,
) -> HttpResponse {
  info!(
      data.logger,"[BLOBUPLOAD.DELETE]";
      "name"=>info.clone().name,
      "uuid"=>info.clone().uuid,
      "digest"=>query.clone().digest
  );
  HttpResponse::Ok().finish()
}
