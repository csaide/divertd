// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// macro includes
#[macro_use]
extern crate slog;
#[macro_use]
extern crate clap;

// extern usings
use common::log;
use structopt::StructOpt;

// local mods
mod rest;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    global_settings = &[clap::AppSettings::DeriveDisplayOrder],
    author = "Christian Saide <me@csaide.dev>",
    about = "Redirect and URL shortening service for divertd.dev"
)]
struct Config {
    #[structopt(flatten)]
    log_config: log::Config,
}

pub fn run() -> i32 {
    let cfg = Config::from_args();

    let setup_logger = log::new(
        &log::Config {
            handler: log::Handler::Stdout,
            level: log::Level::Crit,
            path: String::from(""),
        },
        crate_name!(),
        crate_version!(),
    )
    .unwrap();

    let root_logger = match log::new(&cfg.log_config, crate_name!(), crate_version!()) {
        Ok(root_logger) => root_logger,
        Err(e) => {
            crit!(setup_logger, "Failed to generate logger based on supplied configuration."; e);
            return 1;
        }
    };

    match rest::main() {
        Ok(_) => info!(root_logger, "Gracefully shutdown"),
        Err(_) => crit!(root_logger, "Something exploded"),
    };

    info!(root_logger, "Server shutdown.");
    return 0;
}
