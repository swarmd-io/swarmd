// Adapted from `deno`.
// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_core::Snapshot;
use log::debug;

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

pub fn swarmd_isolate_init() -> Snapshot {
    debug!("Swarmd isolate init with snapshots.");
    Snapshot::Static(RUNTIME_SNAPSHOT)
}

pub static SOURCE_CODE_FOR_99_MAIN_JS: &str = include_str!("js/99_main.js");
