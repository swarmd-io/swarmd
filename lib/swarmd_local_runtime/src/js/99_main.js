// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

import * as errors from "ext:runtime/01_errors.ts";
import * as util from "ext:runtime/03_util.ts";
import * as versionSwarmd from "ext:runtime/02_version.ts";

import * as event from 'ext:deno_web/02_event.js';
import * as timers from 'ext:deno_web/02_timers.js';
import * as performance from "ext:deno_web/15_performance.js";
import * as location from "ext:deno_web/12_location.js";

import { swarmdNs } from "ext:runtime/90_swarmd.ts";
import { windowOrWorkerGlobalScope, globalProperties } from "ext:runtime/98_global_scope.ts";

const core = globalThis.Deno.core;
const primordials = globalThis.__bootstrap.primordials;

const {
  ArrayPrototypePush,
  ArrayPrototypeSplice,
  ArrayPrototypeIndexOf,
  ArrayPrototypeShift,
  ObjectDefineProperty,
  ObjectDefineProperties,
  ObjectSetPrototypeOf,
  ObjectFreeze,
  DateNow,
  StringPrototypeSplit,
  SafeWeakMap,
  WeakMapPrototypeSet,
  WeakMapPrototypeDelete,
  WeakMapPrototypeGet,
} = primordials;

// ----------------------------------------------------------------------------
core.registerErrorClass("NotFound", errors.NotFound);
core.registerErrorClass("PermissionDenied", errors.PermissionDenied);
core.registerErrorClass("ConnectionRefused", errors.ConnectionRefused);
core.registerErrorClass("ConnectionReset", errors.ConnectionReset);
core.registerErrorClass("ConnectionAborted", errors.ConnectionAborted);
core.registerErrorClass("NotConnected", errors.NotConnected);
core.registerErrorClass("AddrInUse", errors.AddrInUse);
core.registerErrorClass("AddrNotAvailable", errors.AddrNotAvailable);
core.registerErrorClass("BrokenPipe", errors.BrokenPipe);
core.registerErrorClass("AlreadyExists", errors.AlreadyExists);
core.registerErrorClass("InvalidData", errors.InvalidData);
core.registerErrorClass("TimedOut", errors.TimedOut);
core.registerErrorClass("Interrupted", errors.Interrupted);
core.registerErrorClass("WouldBlock", errors.WouldBlock);
core.registerErrorClass("WriteZero", errors.WriteZero);
core.registerErrorClass("UnexpectedEof", errors.UnexpectedEof);
core.registerErrorClass("BadResource", errors.BadResource);
core.registerErrorClass("Http", errors.Http);
core.registerErrorClass("Busy", errors.Busy);
core.registerErrorClass("NotSupported", errors.NotSupported);
core.registerErrorClass("FilesystemLoop", errors.FilesystemLoop);
core.registerErrorClass("IsADirectory", errors.IsADirectory);
core.registerErrorClass("NetworkUnreachable", errors.NetworkUnreachable);
core.registerErrorClass("NotADirectory", errors.NotADirectory);
core.registerErrorBuilder(
  "DOMExceptionOperationError",
  function DOMExceptionOperationError(msg) {
    return new DOMException(msg, "OperationError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionQuotaExceededError",
  function DOMExceptionQuotaExceededError(msg) {
    return new DOMException(msg, "QuotaExceededError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionNotSupportedError",
  function DOMExceptionNotSupportedError(msg) {
    return new DOMException(msg, "NotSupported");
  },
);
core.registerErrorBuilder(
  "DOMExceptionNetworkError",
  function DOMExceptionNetworkError(msg) {
    return new DOMException(msg, "NetworkError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionAbortError",
  function DOMExceptionAbortError(msg) {
    return new DOMException(msg, "AbortError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionInvalidCharacterError",
  function DOMExceptionInvalidCharacterError(msg) {
    return new DOMException(msg, "InvalidCharacterError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionDataError",
  function DOMExceptionDataError(msg) {
    return new DOMException(msg, "DataError");
  },
);
// ----------------------------------------------------------------------------

let bootstraped = false;
// To access globalThis even if it's deleted.
let globalThis_;

// Remove Intl.v8BreakIterator because it is a non-standard API.
delete Intl.v8BreakIterator;

/**
 * Clear function to remove useless stuff on the globalThis scope
 */
function endProcess() {
  // Removes the `__proto__` for security reasons.
  // https://tc39.es/ecma262/#sec-get-object.prototype.__proto__
  delete Object.prototype.__proto__;

  // We don't want to have this accessible, even if those functions are
  // available.
  // delete globalThis.Deno;

  // Remove `eval` for security reasons.
  Object.defineProperty(globalThis, 'eval', {
    value: undefined,
    writable: false,
    configurable: false
  });

  // Remove `WebAssembly` because we won't support it for now.
  Object.defineProperty(globalThis, 'WebAssembly', {
    value: undefined,
    writable: false,
    configurable: false
  });

  // Remove bootstrapping data from the global scope
  delete globalThis.__bootstrap;
  delete globalThis.bootstrap;
}

const pendingRejections = [];
const pendingRejectionsReasons = new SafeWeakMap();

function promiseRejectCallback(type, promise, reason) {
  switch (type) {
    case 0: {
      ops.op_store_pending_promise_rejection(promise, reason);
      ArrayPrototypePush(pendingRejections, promise);
      WeakMapPrototypeSet(pendingRejectionsReasons, promise, reason);
      break;
    }
    case 1: {
      ops.op_remove_pending_promise_rejection(promise);
      const index = ArrayPrototypeIndexOf(pendingRejections, promise);
      if (index > -1) {
        ArrayPrototypeSplice(pendingRejections, index, 1);
        WeakMapPrototypeDelete(pendingRejectionsReasons, promise);
      }
      break;
    }
    default:
      return false;
  }

  return !!globalThis_.onunhandledrejection ||
    event.listenerCount(globalThis_, "unhandledrejection") > 0 ||
    typeof internals.nodeProcessUnhandledRejectionCallback !== "undefined";
}

function promiseRejectMacrotaskCallback() {
  // We have no work to do, tell the runtime that we don't
  // need to perform microtask checkpoint.
  if (pendingRejections.length === 0) {
    return undefined;
  }

  while (pendingRejections.length > 0) {
    const promise = ArrayPrototypeShift(pendingRejections);
    const hasPendingException = ops.op_has_pending_promise_rejection(
      promise,
    );
    const reason = WeakMapPrototypeGet(pendingRejectionsReasons, promise);
    WeakMapPrototypeDelete(pendingRejectionsReasons, promise);

    if (!hasPendingException) {
      continue;
    }

    const rejectionEvent = new event.PromiseRejectionEvent(
      "unhandledrejection",
      {
        cancelable: true,
        promise,
        reason,
      },
    );

    const errorEventCb = (event) => {
      if (event.error === reason) {
        ops.op_remove_pending_promise_rejection(promise);
      }
    };
    // Add a callback for "error" event - it will be dispatched
    // if error is thrown during dispatch of "unhandledrejection"
    // event.
    globalThis_.addEventListener("error", errorEventCb);
    globalThis_.dispatchEvent(rejectionEvent);
    globalThis_.removeEventListener("error", errorEventCb);

    // If event was not yet prevented, try handing it off to Node compat layer
    // (if it was initialized)
    if (
      !rejectionEvent.defaultPrevented &&
      typeof internals.nodeProcessUnhandledRejectionCallback !== "undefined"
    ) {
      internals.nodeProcessUnhandledRejectionCallback(rejectionEvent);
    }

    // If event was not prevented (or "unhandledrejection" listeners didn't
    // throw) we will let Rust side handle it.
    if (rejectionEvent.defaultPrevented) {
      ops.op_remove_pending_promise_rejection(promise);
    }
  }
  return true;
}

/**
 * Function to initiate the whole Worker
 */
function bootstrapWorker(version, target) {
  if (bootstraped) {
    throw new Error("Worker runtime already bootstrapped");
  }

  performance.setTimeOrigin(DateNow());
  globalThis_ = globalThis;
  const ops = globalThis_.Deno.core.ops;

  ObjectDefineProperty(globalThis, 'SWARMD_VERSION', util.readOnly(String(version)));

  bootstraped = true;

  // We set Global Properties
  ObjectDefineProperties(globalThis, globalProperties);

  // Set the location to the provided host name.
  location.setLocationHref("https://local-worker.d.swarmd.net");

  // ---------------------------------------------------------------------

  ObjectSetPrototypeOf(globalThis, Window.prototype);

  event.setEventTargetData(globalThis);
  event.saveGlobalThisReference(globalThis);

  event.defineEventHandler(globalThis, "error");
  event.defineEventHandler(globalThis, "load");
  event.defineEventHandler(globalThis, "beforeunload");
  event.defineEventHandler(globalThis, "unload");
  event.defineEventHandler(globalThis, "unhandledrejection");

  core.setPromiseRejectCallback(promiseRejectCallback);

  // ---------------------------------------------------------------------
  // Runtime initialization
  core.setMacrotaskCallback(timers.handleTimerMacrotask);
  core.setMacrotaskCallback(promiseRejectMacrotaskCallback);

  // We desactivated Wasm for now.
  // core.setWasmStreamingCallback(fetch.handleWasmStreaming);
  core.setReportExceptionCallback(event.reportException);
  ops.op_set_format_exception_callback(util.formatException);
  core.setBuildInfo(target);
  // Error.prepareStackTrace = core.prepareStackTrace;


  // ----------------------------------------------------------------------
  ObjectDefineProperties(globalThis, windowOrWorkerGlobalScope);

  // ----------------------------------------------------------------------
  // Swarmd Namespace
  ObjectDefineProperty(globalThis, "Swarmd", util.readOnly(swarmdNs));
  // ----------------------------------------------------------------------

  // ---------------------------------------------------------------------
  // Overriding Deno Namespace once initialization completed
  // ---------------------------------------------------------------------

  versionSwarmd.setVersions(globalThis.SWARMD_VERSION, '11.6.189.12', '5.1.6');

  const build = {
    target: 'unknown',
    arch: 'unknown',
    os: 'unknown',
    vendor: 'unknown',
    env: undefined,
  };

  function setBuildInfo(target) {
    const { 0: arch, 1: vendor, 2: os, 3: env } = StringPrototypeSplit(
      target,
      '-',
      4,
    );
    build.target = target;
    build.arch = arch;
    build.vendor = vendor;
    build.os = os;
    build.env = env;

    ObjectFreeze(build);
  }
  setBuildInfo(target);

  const denoNs = {};
  ObjectDefineProperties(denoNs, {
    build: util.readOnly(build),
    env: util.readOnly(undefined),
    pid: util.readOnly(1),
    args: util.readOnly([]),
    mainModule: util.getterOnly(() => ops.op_main_module()),
    version: util.getterOnly(() => versionSwarmd.version),
  });
  ObjectDefineProperty(globalThis, 'Deno', util.readOnly(denoNs));

  endProcess();
}


globalThis.bootstrapWorker = bootstrapWorker;

