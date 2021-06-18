use crate::media_types::MediaType;
use crate::{AppState, NameReference};
use actix_web::{delete, get, head, http, put, web, HttpRequest, HttpResponse, Responder};
use bytes::Buf;
use bytes::Bytes;
use sha256::digest as Sha256;

// TODO: get manifest
/// GET Manifest
/// Fetch the manifest identified by name and reference where reference can be a tag or digest.
#[get("/{name:.*}/manifests/{reference}")]
pub async fn get_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
) -> impl Responder {
    info!(
        data.logger,
        "[MANIFEST.DOWNLOAD]";"name"=>&info.name,"reference"=>&info.reference,
    );

    // TODO: if Accept header not include OCI, but Manifest is OCI format, return 404.

    // TODO: get digest
    let digest = "";
    HttpResponse::Ok()
        .header("Content-Type", MediaType::ManifestV2.to_str())
        .header("Docker-Content-Digest", digest)
        .body("{}")
}

// TODO: check manifest
/// HEAD Manifest
/// Check is the manifest is exists.
#[head("/{name:.*}/manifests/{reference}")]
pub async fn head_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
) -> impl Responder {
    info!(
        data.logger,
        "[MANIFEST.CHECK]";"name"=>&info.name,"reference"=>&info.reference,
    );

    // TODO: if Accept header not include OCI, but Manifest is OCI format, return 404.

    let digest = "";
    HttpResponse::Ok()
        .header("Content-Type", MediaType::ManifestV2.to_str())
        .header("Docker-Content-Digest", digest)
        .body("")
}

// TODO:
/// PUT Manifest
/// Put the manifest identified by name and reference where reference can be a tag or digest.
/// Will validates and stores a manifest in the registry.
#[put("/{name:.*}/manifests/{reference}")]
pub async fn put_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
    req: HttpRequest,
    body: Bytes,
) -> impl Responder {
    info!(
        data.logger,
        "[MANIFEST.PUT]";
        "name"=>&info.name,
        "reference"=>&info.reference,
        "body.size"=>body.len(),
    );

    // TODO: optimize - limit read body
    if body.len() > 4194304 {
        HttpResponse::BadRequest();
    }

    let body_str = std::str::from_utf8(body.bytes()).unwrap_or("");
    let content_type_header = req.headers().get(http::header::CONTENT_TYPE);
    // TODO: valid manifest content.
    match content_type_header {
        Some(v) => {
            let content_type = v.to_str().unwrap();
            debug!(data.logger,"has content_type header";"content_type"=>content_type);
            match MediaType::from_str(content_type) {
                // docker manifest
                MediaType::ManifestV1 | MediaType::ManifestV2 | MediaType::ManifestList => (),
                // OCI manifest
                MediaType::ImageManifest | MediaType::ImageIndex => (),
                _ => {
                    error!(data.logger,"bad manifest content type";"content_type"=>content_type);
                    return HttpResponse::BadRequest().body("");
                }
            }
        }
        None => {
            debug!(data.logger,"not content_type header";);
            return HttpResponse::BadRequest().body("");
        }
    }

    let digest = Sha256(body_str);
    debug!(
        data.logger,
        "[MANIFEST.PUT]";"body"=>body_str,"digest"=>digest.clone(),
    );

    HttpResponse::Created()
        .header("Location", "") // TODO: The canonical location url of the uploaded manifest.
        .header("Docker-Content-Digest", format!("sha256:{}", digest))
        .header("Content-Length", "0") // !!! Must be zero
        .body("") // !!! Must be empty
}

// TODO
/// DELETE Manifest
/// Delete the manifest identified by name and reference. Note that a manifest can only be deleted by digest.
#[delete("/{name:.*}/manifests/{reference}")]
pub async fn delete_manifest() -> impl Responder {
    HttpResponse::Accepted().body("")
}
