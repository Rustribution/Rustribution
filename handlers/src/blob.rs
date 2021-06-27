use crate::DOCKER_CONTENT_DIGEST;
use crate::{build_blob_path, AppState, NameDigest};
use actix_files::HttpRange;
use actix_web::{delete, get, head, http, web, HttpRequest, HttpResponse, Responder};
use std::io::ErrorKind::NotFound;

/// Fetch Blob or Part
/// Retrieve the blob from the registry identified by digest.
#[get("/{name:.*}/blobs/{digest}")]
pub async fn fetch_blob(
    data: web::Data<AppState>,
    info: web::Path<NameDigest>,
    req: HttpRequest,
) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.FETCH]";"name"=>&info.name,"digest"=>&info.digest,
    );

    let path = build_blob_path(info.digest.clone());
    let range_header = req.headers().get(http::header::RANGE);
    match range_header {
        Some(v) => {
            // TODO: Fetch Blob Part
            let vstr = v.to_str().unwrap();
            let layer_size = 1024000; // TODO
            let range = HttpRange::parse(vstr, layer_size).unwrap()[0];
            info!(data.logger, "BLOB.FETCH.PART";"name"=>&info.name,"digest"=>&info.digest,
            "range_header"=>vstr, "range.start"=>range.start,"range.length"=>range.length);
            HttpResponse::PartialContent()
                .header("Content-Length", range.length) // TODO: Length of body
                .header(
                    "Content-Range",
                    format!(
                        "bytes {}-{}/{}",
                        range.start,
                        range.start + range.length,
                        layer_size
                    ),
                )
                .content_type("application/octet-stream")
                .finish()
        }
        None => {
            match data.backend.get_content(path.clone()) {
                Ok(content) => {
                    HttpResponse::Ok()
                        .header("Content-Length", "0") // TODO: Length of body
                        .content_type("application/octet-stream")
                        .header(DOCKER_CONTENT_DIGEST, info.digest.clone())
                        .body(content)
                }
                Err(e) => match e.kind() {
                    NotFound => {
                        warn!(
                            data.logger,
                            "get blob failed";
                            "error"=> &e,
                            "path"=> path,
                        );

                        HttpResponse::NotFound()
                            .header(DOCKER_CONTENT_DIGEST, info.digest.clone())
                            .finish()
                    }
                    _ => {
                        error!(
                            data.logger,
                            "get blob failed";
                            "error"=> &e,
                            "path"=> path,
                        );

                        HttpResponse::InternalServerError().finish()
                    }
                },
            }
        }
    }
}

/// Check Blob
/// Check the blob from the registry identified by digest.
#[head("/{name:.*}/blobs/{digest}")]
pub async fn check_blob(
    data: web::Data<AppState>,
    info: web::Path<NameDigest>,
    _payload: web::Payload,
) -> impl Responder {
    let path = build_blob_path(info.digest.clone());

    match data.backend.stat(path) {
        Ok(size) => {
            info!(
                data.logger,
                "[BLOB.CHECK]";
                "name"=>&info.name,
                "digest"=>&info.digest,
                "size"=>size,
            );
            HttpResponse::Ok()
                .header(DOCKER_CONTENT_DIGEST, info.digest.to_string())
                .header("Etag", format!(r#""{}""#, info.digest.to_string()))
                .header("Accept-Ranges", "bytes")
                .content_type("application/octet-stream")
                .no_chunking(size as u64)
                .streaming(_payload)
        }
        Err(e) => match e.kind() {
            NotFound => {
                warn!(
                    data.logger,
                    "blob not found";
                    "error"=> e,
                );

                HttpResponse::NotFound().finish()
            }
            _ => {
                error!(
                    data.logger,
                    "check blob failed";
                    "digest"=>info.digest.to_string(),
                    "error"=> e,
                );

                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

/// DELETE Blob
/// Delete the blob from the registry identified by digest.
#[delete("/{name:.*}/blobs/{digest}")]
pub async fn delete_blob(data: web::Data<AppState>, info: web::Path<NameDigest>) -> impl Responder {
    // TODO:
    let path = build_blob_path(info.digest.clone());
    let backend = &data.backend;

    match backend.delete(path.clone()) {
        Ok(_) => {
            info!(
                data.logger,
                "[BLOB.DELETE]";
                "name"=>&info.name,
                "digest"=>&info.digest,
                "path"=>path.clone(),
            );
            return HttpResponse::Accepted()
                .header(DOCKER_CONTENT_DIGEST, info.digest.clone())
                .finish();
        }
        Err(e) => match e.kind() {
            NotFound => {
                warn!(
                    data.logger,
                    "remove blob failed";
                    "name"=>&info.name,
                    "digest"=>&info.digest,
                    "path"=>path,
                    "error"=> e,
                );
                return HttpResponse::NotFound().finish();
            }
            _ => {
                error!(
                    data.logger,
                    "remove blob failed";
                    "name"=>&info.name,
                    "digest"=>&info.digest,
                    "path"=>path,
                    "error"=> e,
                );

                return HttpResponse::InternalServerError().finish();
            }
        },
    };
}
