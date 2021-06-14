#[macro_use(crate_authors, crate_version, crate_name, crate_description)]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_term;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate std;

mod config;
mod handlers;
mod options;
mod slogger;

use actix_slog::StructuredLogger;
use actix_web::{middleware::Compress, web, App, HttpServer};
use handlers::{
    backend_info, check_blob, check_manifest, download_manifest, init_upload, monolithic_upload,
    status_upload, upload_manifest, v2, AppState,
};
use options::Options;
use storage::factory as StorageFactory;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Options::from_args();
    let config = config::parse(opt.config).unwrap();

    let logger = slogger::init();
    let location_logger = slogger::with_location(logger.clone());
    debug!(location_logger,"";"storage.backend_type"=>config.clone().storage.backend_type);

    let backend = StorageFactory::new_backend(config.clone().storage, location_logger.clone())?;
    info!(location_logger, "backend info {}", backend.info());

    let cfg = config.clone();
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                logger: location_logger.clone(),
                config: config.clone(),
                backend: backend.clone(),
            })
            .data(config.clone())
            .wrap(Compress::default())
            .wrap(StructuredLogger::new(logger.new(o!())))
            .service(
                web::scope("/v2")
                    .service(check_blob)
                    .service(init_upload)
                    .service(status_upload)
                    .service(upload_manifest)
                    .service(monolithic_upload)
                    .service(download_manifest)
                    .service(check_manifest)
                    .service(backend_info)
                    .route("/", web::to(v2)),
            )
    })
    .bind(cfg.http.addr)?
    .run()
    .await
}
