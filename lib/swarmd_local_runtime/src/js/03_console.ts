// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Objet utility functions

const core = globalThis.Deno.core;
const ops = core.ops;
const primordials = globalThis.__bootstrap.primordials;

const {
  SafeArrayIterator,
} = primordials;

// WARNING: Keep this in sync with Rust:  `bootstrap::WorkerLogLevel`
const LogLevel = {
  Error: 1,
  Warn: 2,
  Info: 3,
  Debug: 4,
};

const logSource = "JS";

let logLevel_: number | null = null;

/** return the log level */
function logLevel(): number {
  if (logLevel_ === null) {
    logLevel_ = ops.op_bootstrap_log_level() || 3;
  }
  return logLevel_;
}

function log(...args: any[]) {
  if (logLevel() >= LogLevel.Debug) {
    // if we destructure `console` off `globalThis` too early, we don't bind to
    // the right console, therefore we don't log anything out.
    globalThis.console.error(
      `DEBUG ${logSource} -`,
      ...new SafeArrayIterator(args),
    );
  }
}
export { log };
