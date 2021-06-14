use slog::*;
// use slog::{fuse, FnValue, PushFnValue};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};
use std::sync::Arc;

pub fn init() -> Logger {
  let decorator = TermDecorator::new().build();
  let drain = FullFormat::new(decorator).build().fuse();
  let drain = Async::new(drain).build().fuse();

  let logger = Logger::root(
    Arc::new(drain),
    o!(
      "module" => FnValue(move |info| {
            info.module().to_string()
        }),
      "pkg_version"=>crate_version!(),
    ),
  );

  logger
}

/// Record source code name and line.
pub fn with_location(logger: Logger) -> Logger {
  Logger::root(
    logger,
    o!(
       "location"=>PushFnValue(move |r,ser|{
            ser.emit(format_args!("{}:{}", r.file(), r.line()))
          }),
    ),
  )
}

// /// Record log_type
// pub fn with_logtype(logger: Logger, logtype: String) -> Logger {
//   Logger::root(logger, o!("logtype", logger))
// }
