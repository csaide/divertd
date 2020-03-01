// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// super usings
use super::error::{Error, Result};

// stdlib usings
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Level {
    Crit,
    Error,
    Warn,
    Info,
    Debug,
}

impl FromStr for Level {
    type Err = Error;

    fn from_str(t: &str) -> Result<Level> {
        match t {
            "critical" => Ok(Level::Crit),
            "error" => Ok(Level::Error),
            "warn" => Ok(Level::Warn),
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            _ => Err(Error::InvalidLevel {
                level: t.to_owned(),
            }),
        }
    }
}

impl Level {
    pub fn to_slog(&self) -> slog::Level {
        match self {
            Level::Crit => slog::Level::Critical,
            Level::Error => slog::Level::Error,
            Level::Warn => slog::Level::Warning,
            Level::Info => slog::Level::Info,
            Level::Debug => slog::Level::Debug,
        }
    }
}
