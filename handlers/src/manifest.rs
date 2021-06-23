use crate::build_blob_path;
use crate::media_types::MediaType;
use crate::{AppState, NameReference};
use actix_web::{delete, get, head, http, put, web, HttpRequest, HttpResponse, Responder};
use bytes::Buf;
use bytes::Bytes;
use sha256::digest as Sha256;

/// GET Manifest
/// Fetch the manifest identified by name and reference where reference can be a tag or digest.
#[get("/{name:.*}/manifests/{reference}")]
pub async fn get_manifest(
    data: web::Data<AppState>,
    info: web::Path<NameReference>,
) -> impl Responder {
    let mut digest: String = String::new();
    if info.reference.len() == 71 && &info.reference[0..7].as_ref() == b"sha256:" {
        digest = info.reference.clone();
    } else {
        // TODO: get digest by tag
    }

    info!(
        data.logger,
        "[MANIFEST.DOWNLOAD]";
        "name"=>&info.name,
        "reference"=>&info.reference,
        "reference.length"=>&info.reference.len(),
        "digest"=>digest.clone(),
        "info.reference.len()"=>info.reference.len(),
    );

    // TODO: if Accept header not include OCI, but Manifest is OCI format, return 404.

    let blobpath = build_blob_path(digest.clone());
    let backend = data.backend.lock().unwrap();

    let (exsit, _) = backend.stat(blobpath.clone());
    if !exsit {
        return HttpResponse::NotFound()
            .header("Docker-Content-Digest", digest.clone())
            .finish();
    }

    let content = backend.get_content(blobpath.clone());
    debug!(
        data.logger,"";
        "blobpath"=>blobpath.clone(),
        "digest"=>digest.clone(),
        "content.length"=>content.len(),
    );

    HttpResponse::Ok()
        .header("Content-Type", MediaType::ManifestV2.to_str())
        .header("Docker-Content-Digest", digest)
        .body(content)
}

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

    let digest = format!("sha256:{}", Sha256(body_str));
    data.backend
        .lock()
        .unwrap()
        .put_content(build_blob_path(digest.clone()), body.clone());

    debug!(
        data.logger,
        "[MANIFEST.PUT]";"body"=>body_str,"digest"=>digest.clone(),
    );

    HttpResponse::Created()
        .header("Location", "") // TODO: The canonical location url of the uploaded manifest.
        .header("Docker-Content-Digest", digest)
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
