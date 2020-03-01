// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// super usings
use super::error::{Error, Result};

// stdlib usings
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Handler {
    File,
    Stdout,
}

impl FromStr for Handler {
    type Err = Error;

    fn from_str(t: &str) -> Result<Handler> {
        match t {
            "file" => Ok(Handler::File),
            "stdout" => Ok(Handler::Stdout),
            _ => Err(Error::HandlerMissing {
                handler: t.to_owned(),
            }),
        }
    }
}
