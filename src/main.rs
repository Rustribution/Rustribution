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
mod options;
mod slogger;

use actix_slog::StructuredLogger;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use handlers::base::v2;
use handlers::blob::{check_blob, delete_blob, fetch_blob};
use handlers::init_blob_upload::init_upload;
use handlers::manifest::{delete_manifest, get_manifest, head_manifest, put_manifest};
use handlers::tags::tags_list;
use handlers::{AppState, DISTRIBUTION_API_VERSION, RUSTRIBUTION_VERSION};
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

    // default metrics
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint(&config.clone().http.prometheus.unwrap().path)
        .build()
        .unwrap();

    let cfg = config.clone();
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                logger: location_logger.clone(),
                // config: config.clone(),
                backend: backend.clone(),
            })
            .data(config.clone())
            .wrap(middleware::Condition::new(
                config.clone().http.prometheus.unwrap().enabled,
                prometheus.clone(),
            ))
            .wrap(StructuredLogger::new(logger.new(o!())))
            .wrap(
                middleware::DefaultHeaders::default()
                    .header(DISTRIBUTION_API_VERSION, "registry/2.0")
                    .header(RUSTRIBUTION_VERSION, crate_version!()),
            )
            .service(
                web::scope("/v2")
                    // tags
                    .service(tags_list)
                    // manifest
                    .service(get_manifest)
                    .service(head_manifest)
                    .service(put_manifest)
                    .service(delete_manifest)
                    // blob
                    .service(check_blob)
                    .service(fetch_blob)
                    .service(delete_blob)
                    // init upload
                    .service(init_upload)
                    // TODO: upload hanlers
                    //
                    .route("/", web::to(v2)),
            )
    })
    .bind(cfg.http.addr)?
    .run()
    .await
}
