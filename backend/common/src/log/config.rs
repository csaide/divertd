// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// super usings
use super::handler::Handler;
use super::level::Level;

// extern usings
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(
        long = "log-handler",
        short = "t",
        env = "LOG_HANDLER",
        help = "The logging handler to use.",
        default_value = "stdout",
        possible_values = &["stdout", "file"],
        takes_value = true
    )]
    pub handler: Handler,

    #[structopt(
        long = "log-level",
        short = "l",
        env = "LOG_LEVEL",
        help = "The logging level to use.",
        default_value = "info",
        possible_values = &["critical", "error", "warn", "info", "debug"],
        takes_value = true
    )]
    pub level: Level,

    #[structopt(
        long = "log-file",
        short = "f",
        env = "LOG_FILE",
        help = "The log file to write to if the 'file' log handler is used.",
        default_value = "",
        required_if("log_handler", "file"),
        takes_value = true
    )]
    pub path: String,
}
