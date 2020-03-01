// (c) Copyright 2020 Christian Saide <supernomad>
// SPDX-License-Identifier: GPL-3.0-only

// stdlib usings
use std::process;

fn main() {
    let ret = divertd::run();
    process::exit(ret);
}
