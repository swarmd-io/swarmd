// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Objet utility functions
import * as console from "ext:deno_console/01_console.js";

const core = globalThis.Deno.core;
const ops = core.ops;
const primordials = globalThis.__bootstrap.primordials;

const {
  Promise,
  ObjectPrototypeIsPrototypeOf,
  ErrorPrototype,
} = primordials;

interface Promise<T> {
  resolve: undefined | ((value: T | PromiseLike<T>) => void);
  reject: undefined | ((reason?: any) => void);
}

function createResolvable<T>(): Promise<T> {
  let resolve: (value: T | PromiseLike<T>) => void;
  let reject: (reason?: any) => void;

  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });

  // @ts-ignore
  promise.resolve = resolve;
  // @ts-ignore
  promise.reject = reject;

  // @ts-ignore
  return promise;
}

function writable<T>(value: T) {
  return {
    value,
    writable: true,
    enumerable: true,
    configurable: true,
  };
}

function nonEnumerable<T>(value: T) {
  return {
    value,
    writable: true,
    enumerable: false,
    configurable: true,
  };
}

function readOnly<T>(value: T) {
  return {
    value,
    enumerable: true,
    writable: false,
    configurable: true,
  };
}

function getterOnly<T>(getter: T) {
  return {
    get: getter,
    set() { },
    enumerable: true,
    configurable: true,
  };
}

function formatException(error: any) {
  if (ObjectPrototypeIsPrototypeOf(ErrorPrototype, error)) {
    return null;
  } else if (typeof error == "string") {
    return `Uncaught ${console.inspectArgs([console.quoteString(error, { colors: false })], {
      colors: false,
    })
      }`;
  } else {
    return `Uncaught ${console.inspectArgs([error], { colors: false })}`;
  }
}


export { createResolvable, getterOnly, nonEnumerable, readOnly, writable, formatException };
