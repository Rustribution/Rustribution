use crate::{AppState, NameDigest};
use actix_files::HttpRange;
use actix_web::{delete, get, head, http, web, HttpRequest, HttpResponse, Responder};

// TODO
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
            // Fetch Blob Part
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
            HttpResponse::Ok()
                .header("Content-Length", "0") // TODO: Length of body
                .header("Content-Type", "application/octet-stream")
                .header("Docker-Content-Digest", info.digest.clone())
                .body("")
        }
    }
}

// TODO
/// Check Blob
/// Check the blob from the registry identified by digest.
#[head("/{name:.*}/blobs/{digest}")]
pub async fn check_blob(data: web::Data<AppState>, info: web::Path<NameDigest>) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.CHECK]";"name"=>&info.name,"digest"=>&info.digest,
    );

    HttpResponse::Ok()
        .header("Docker-Content-Digest", info.digest.to_string())
        .body("")
}

// TODO
/// DELETE Blob
/// Delete the blob from the registry identified by digest.
#[delete("/{name:.*}/blobs/{digest}")]
pub async fn delete_blob(data: web::Data<AppState>, info: web::Path<NameDigest>) -> impl Responder {
    info!(
        data.logger,
        "[BLOB.CHECK]";"name"=>&info.name,"digest"=>&info.digest,
    );

    let digest = ""; // TODO
    HttpResponse::Accepted()
        .header("Docker-Content-Digest", digest)
        .body("")
}
