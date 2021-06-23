use crate::{build_blob_path, AppState, NameDigest};
use actix_files::HttpRange;
use actix_web::{delete, get, head, http, web, HttpRequest, HttpResponse, Responder};

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
                .header("Content-Type", "application/octet-stream")
                .body("")
        }
        None => {
            let content = data
                .backend
                .lock()
                .unwrap()
                .get_content(build_blob_path(info.digest.clone()));
            HttpResponse::Ok()
                .header("Content-Length", "0") // TODO: Length of body
                .header("Content-Type", "application/octet-stream")
                .header("Docker-Content-Digest", info.digest.clone())
                .body(content)
        }
    }
}

/// Check Blob
/// Check the blob from the registry identified by digest.
#[head("/{name:.*}/blobs/{digest}")]
pub async fn check_blob(
    data: web::Data<AppState>,
    info: web::Path<NameDigest>,
    payload: web::Payload,
) -> impl Responder {
    let path = build_blob_path(info.digest.clone());
    let (exist, size) = data.backend.lock().unwrap().stat(path);
    info!(
        data.logger,
        "[BLOB.CHECK]";
        "name"=>&info.name,
        "digest"=>&info.digest,
        "exist"=>exist,
        "size"=>size,
    );

    if exist {
        // It used for put manifest.
        HttpResponse::Ok()
            .header("Docker-Content-Digest", info.digest.to_string())
            .header("Etag", format!(r#""{}""#, info.digest.to_string()))
            .content_type("application/octet-stream")
            .no_chunking(size as u64)
            .streaming(payload)
    } else {
        HttpResponse::NotFound().finish()
    }
}

/// DELETE Blob
/// Delete the blob from the registry identified by digest.
#[delete("/{name:.*}/blobs/{digest}")]
pub async fn delete_blob(data: web::Data<AppState>, info: web::Path<NameDigest>) -> impl Responder {
    // TODO:
    let path = build_blob_path(info.digest.clone());
    let backend = data.backend.lock().unwrap();
    let (exist, _) = backend.stat(path.clone());

    info!(
        data.logger,
        "[BLOB.DELETE]";
        "name"=>&info.name,
        "digest"=>&info.digest,
        "exist"=>exist.clone(),
        "path"=>path.clone(),
    );

    if !exist {
        return HttpResponse::NotFound().finish();
    }
    backend.delete(path);
    HttpResponse::Accepted()
        .header("Docker-Content-Digest", info.digest.clone())
        .finish()
}
