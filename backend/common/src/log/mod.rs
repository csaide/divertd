// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// stdlib usings
use std::fs::OpenOptions;
use std::io;

// extern usings
use slog::Drain;

// local modules
mod config;
mod error;
mod filter;
mod handler;
mod level;

// local module exports
pub use self::config::Config;
pub use self::error::{Error, Result};
pub use self::handler::Handler;
pub use self::level::Level;

pub fn new(
    cfg: &config::Config,
    app: &'static str,
    version: &'static str,
) -> error::Result<slog::Logger> {
    match cfg.handler {
        Handler::File => {
            let file = match OpenOptions::new().create(true).append(true).open(&cfg.path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(error::Error::IOError {
                        path: cfg.path.clone(),
                        err: std::sync::Arc::new(e),
                    });
                }
            };

            let drain = slog_json::Json::new(file).add_default_keys().build().fuse();
            let drain = filter::LevelFilter {
                drain: drain,
                level: cfg.level.to_slog(),
            }
            .fuse();
            let drain = slog_async::Async::new(drain).build().fuse();

            Ok(slog::Logger::root(
                drain,
                o!("app" => app, "ver" => version),
            ))
        }
        Handler::Stdout => {
            let drain = slog_json::Json::new(io::stdout())
                .add_default_keys()
                .build()
                .fuse();
            let drain = filter::LevelFilter {
                drain: drain,
                level: cfg.level.to_slog(),
            }
            .fuse();

            let drain = slog_async::Async::new(drain).build().fuse();

            Ok(slog::Logger::root(
                drain,
                o!("app" => app, "ver" => version),
            ))
        }
    }
}
