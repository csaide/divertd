// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// stdlib usings
use std::result;

// extern usings
use slog::Drain;

pub struct LevelFilter<D> {
    pub drain: D,
    pub level: slog::Level,
}

impl<D> Drain for LevelFilter<D>
where
    D: Drain,
{
    type Err = Option<D::Err>;
    type Ok = Option<D::Ok>;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        if record.level().is_at_least(self.level) {
            self.drain.log(record, values).map(Some).map_err(Some)
        } else {
            Ok(None)
        }
    }
}
