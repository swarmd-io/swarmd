// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
import * as util from "ext:runtime/03_util.ts";
import { Navigator, navigator } from "ext:runtime/04_navigator.js";

import * as abortSignal from 'ext:deno_web/03_abort_signal.js';
import * as location from "ext:deno_web/12_location.js";
import * as base64 from 'ext:deno_web/05_base64.js';
import * as console from 'ext:deno_console/01_console.js';
import DOMException from 'ext:deno_web/01_dom_exception.js';
import * as encoding from 'ext:deno_web/08_text_encoding.js';
import * as event from 'ext:deno_web/02_event.js';
import * as streams from 'ext:deno_web/06_streams.js';
import * as timers from 'ext:deno_web/02_timers.js';
import * as url from 'ext:deno_url/00_url.js';
import * as urlPattern from 'ext:deno_url/01_urlpattern.js';
import * as webidl from 'ext:deno_webidl/00_webidl.js';
import * as performance from "ext:deno_web/15_performance.js";
import * as messagePort from "ext:deno_web/13_message_port.js";

import * as compression from "ext:deno_web/14_compression.js";
import * as crypto from 'ext:deno_crypto/00_crypto.js';
import * as fetch from 'ext:deno_fetch/26_fetch.js';
import * as file from 'ext:deno_web/09_file.js';
import * as fileReader from 'ext:deno_web/10_filereader.js';
import * as formData from 'ext:deno_fetch/21_formdata.js';
import * as headers from 'ext:deno_fetch/20_headers.js';
import * as webSocket from 'ext:deno_websocket/01_websocket.js';
import * as response from 'ext:deno_fetch/23_response.js';
import * as request from 'ext:deno_fetch/23_request.js';
import * as globalInterfaces from 'ext:deno_web/04_global_interfaces.js';
import * as webSocketStream from "ext:deno_websocket/02_websocketstream.js";
import * as eventSource from "ext:deno_fetch/27_eventsource.js";

const unused = { webSocketStream };

const core = globalThis.Deno.core;

// https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope
const windowOrWorkerGlobalScope = {
  EventSource: util.writable(eventSource.EventSource),
  WebSocket: util.nonEnumerable(webSocket.WebSocket),
  // ------------------------
  Headers: util.nonEnumerable(headers.Headers),
  // ------------------------
  FormData: util.nonEnumerable(formData.FormData),
  // ------------------------
  Blob: util.nonEnumerable(file.Blob),
  File: util.nonEnumerable(file.File),
  // ------------------------
  FileReader: util.nonEnumerable(fileReader.FileReader),
  // ------------------------
  Request: util.nonEnumerable(request.Request),
  Response: util.nonEnumerable(response.Response),
  fetch: util.writable(fetch.fetch),
  // ------------------------
  CryptoKey: util.nonEnumerable(crypto.CryptoKey),
  crypto: util.readOnly(crypto.crypto),
  Crypto: util.nonEnumerable(crypto.Crypto),
  SubtleCrypto: util.nonEnumerable(crypto.SubtleCrypto),
  // ------------------------
  CompressionStream: util.nonEnumerable(compression.CompressionStream),
  DecompressionStream: util.nonEnumerable(compression.DecompressionStream),
  // ------------------------
  CloseEvent: util.nonEnumerable(event.CloseEvent),
  CustomEvent: util.nonEnumerable(event.CustomEvent),
  ErrorEvent: util.nonEnumerable(event.ErrorEvent),
  Event: util.nonEnumerable(event.Event),
  EventTarget: util.nonEnumerable(event.EventTarget),
  MessageEvent: util.nonEnumerable(event.MessageEvent),
  PromiseRejectionEvent: util.nonEnumerable(event.PromiseRejectionEvent),
  ProgressEvent: util.nonEnumerable(event.ProgressEvent),
  reportError: util.writable(event.reportError),
  // ------------------------
  clearInterval: util.writable(timers.clearInterval),
  clearTimeout: util.writable(timers.clearTimeout),
  setInterval: util.writable(timers.setInterval),
  setTimeout: util.writable(timers.setTimeout),
  // ------------------------
  MessagePort: util.nonEnumerable(messagePort.MessagePort),
  structuredClone: util.writable(messagePort.structuredClone),
  // ------------------------
  Performance: util.nonEnumerable(performance.Performance),
  PerformanceEntry: util.nonEnumerable(performance.PerformanceEntry),
  PerformanceMark: util.nonEnumerable(performance.PerformanceMark),
  PerformanceMeasure: util.nonEnumerable(performance.PerformanceMeasure),
  performance: util.writable(performance.performance),
  // ------------------------
  AbortController: util.nonEnumerable(abortSignal.AbortController),
  AbortSignal: util.nonEnumerable(abortSignal.AbortSignal),
  // ------------------------
  ByteLengthQueuingStrategy: util.nonEnumerable(streams.ByteLengthQueuingStrategy),
  CountQueuingStrategy: util.nonEnumerable(streams.CountQueuingStrategy),
  ReadableStream: util.nonEnumerable(streams.ReadableStream),
  ReadableStreamDefaultReader: util.nonEnumerable(streams.ReadableStreamDefaultReader),
  TransformStream: util.nonEnumerable(streams.TransformStream),
  WritableStream: util.nonEnumerable(streams.WritableStream),
  WritableStreamDefaultWriter: util.nonEnumerable(streams.WritableStreamDefaultWriter),
  WritableStreamDefaultController: util.nonEnumerable(streams.WritableStreamDefaultController),
  ReadableByteStreamController: util.nonEnumerable(streams.ReadableByteStreamController),
  ReadableStreamBYOBReader: util.nonEnumerable(streams.ReadableStreamBYOBReader),
  ReadableStreamBYOBRequest: util.nonEnumerable(streams.ReadableStreamBYOBRequest),
  ReadableStreamDefaultController: util.nonEnumerable(streams.ReadableStreamDefaultController),
  TransformStreamDefaultController: util.nonEnumerable(streams.TransformStreamDefaultController),
  // ----------------------------------
  TextDecoder: util.nonEnumerable(encoding.TextDecoder),
  TextEncoder: util.nonEnumerable(encoding.TextEncoder),
  TextDecoderStream: util.nonEnumerable(encoding.TextDecoderStream),
  TextEncoderStream: util.nonEnumerable(encoding.TextEncoderStream),
  // ------------------------
  console: util.nonEnumerable(
    new console.Console((msg: string, level: number) => core.print(msg, level > 1)),
  ),
  // ------------------------
  DOMException: util.nonEnumerable(DOMException),
  // ------------------------
  atob: util.writable(base64.atob),
  btoa: util.writable(base64.btoa),
  // ------------------------
  URL: util.nonEnumerable(url.URL),
  URLPattern: util.nonEnumerable(urlPattern.URLPattern),
  URLSearchParams: util.nonEnumerable(url.URLSearchParams),
  // ------------------------
  // Branding as a WebIDL object
  [webidl.brand]: util.nonEnumerable(webidl.brand),
}

const globalProperties = {
  Location: location.locationConstructorDescriptor,
  location: location.locationDescriptor,
  Window: globalInterfaces.windowConstructorDescriptor,
  Window: globalInterfaces.windowConstructorDescriptor,
  window: util.getterOnly(() => globalThis),
  self: util.getterOnly(() => globalThis),
  Navigator: util.nonEnumerable(Navigator),
  navigator: util.getterOnly(() => navigator),
};

export { windowOrWorkerGlobalScope, globalProperties }
