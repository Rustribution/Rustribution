use crate::{AppState, NameUUID, QueryDigest, DOCKER_UPLOAD_UUID};
use actix_web::{delete, get, http, patch, put, web, HttpResponse};

// TODO
/// GET Blob Upload
/// Retrieve status of upload identified by uuid. The primary purpose of this endpoint is to resolve the current status of a resumable upload.
/// The Content-Length header must be zero and the body must be empty.
#[get("/v2/{name:.*}/blobs/uploads/{uuid}")]
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
#[patch("/v2/{name:.*}/blobs/uploads/{uuid}")]
pub async fn stream_upload(data: web::Data<AppState>, info: web::Path<NameUUID>) -> HttpResponse {
  info!(
      data.logger,"[BLOBUPLOAD.CHUNK]";
      "name"=>info.clone().name,
      "uuid"=>info.clone().uuid,
  );
  HttpResponse::Accepted().finish()
}

// TODO
/// PUT Blob Upload
/// Complete the upload specified by uuid, optionally appending the body as the final chunk.
#[put("/v2/{name:.*}/blobs/uploads/{uuid}")]
pub async fn finish_upload(
  data: web::Data<AppState>,
  info: web::Path<NameUUID>,
  query: web::Query<QueryDigest>,
) -> HttpResponse {
  info!(
      data.logger,"[BLOBUPLOAD.FINISH]";
      "name"=>info.clone().name,
      "uuid"=>info.clone().uuid,
      "digest"=>query.clone().digest
  );
  HttpResponse::Created().finish()
}

// TODO
/// DELETE Blob Upload
/// Cancel outstanding upload processes, releasing associated resources. If this is not called, the unfinished uploads will eventually timeout.
#[delete("/v2/{name:.*}/blobs/uploads/{uuid}")]
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
