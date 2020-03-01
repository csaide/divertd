// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// stdlib usings
use std::{error, fmt, io, result};

// extern usings
use serde::ser::SerializeStruct;

static KIND: &str = "log-error";

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, SerdeValue)]
pub enum Error {
    IOError {
        path: String,
        err: std::sync::Arc<io::Error>,
    },
    HandlerMissing {
        handler: String,
    },
    InvalidLevel {
        level: String,
    },
}

impl Error {
    fn message(&self) -> &str {
        match self {
            Error::IOError { .. } => "Could not open configured log path",
            Error::HandlerMissing { .. } => "Specified log handler is invalid",
            Error::InvalidLevel { .. } => "Specified log level is invalid",
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sv = match self {
            Error::IOError { ref path, ref err } => {
                let mut sv = serializer.serialize_struct("Error", 4)?;
                sv.serialize_field("path", path)?;
                sv.serialize_field("cause", &format!("{}", err))?;
                sv
            }
            Error::HandlerMissing { ref handler } => {
                let mut sv = serializer.serialize_struct("Error", 3)?;
                sv.serialize_field("handler", handler)?;
                sv
            }
            Error::InvalidLevel { ref level } => {
                let mut sv = serializer.serialize_struct("Error", 3)?;
                sv.serialize_field("level", level)?;
                sv
            }
        };

        sv.serialize_field("kind", KIND)?;
        sv.serialize_field("description", self.message())?;
        sv.end()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOError { ref path, ref err } => {
                write!(f, "{} '{}': {}", self.message(), path, err)
            }
            Error::HandlerMissing { ref handler } => {
                write!(f, "{}: '{}' is not implemented", self.message(), handler)
            }
            Error::InvalidLevel { ref level } => {
                write!(f, "{}: '{}' is not implemented", self.message(), level)
            }
        }
    }
}

impl slog::KV for Error {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("error", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        KIND
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
