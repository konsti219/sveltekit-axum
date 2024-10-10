import { op_log, op_sleep } from "ext:core/ops";

async function sleep(millis) {
    await op_sleep(millis);
}

globalThis.Extension = { sleep };

//! deno_console

import * as console from "ext:deno_console/01_console.js";

Object.defineProperty(globalThis, "console", {
    value: new console.Console(op_log),
    configurable: true,
    writable: true,
});

//! deno_webidl

import * as webidl from "ext:deno_webidl/00_webidl.js";

Object.defineProperty(globalThis, webidl.brand, {
    value: webidl.brand,
    configurable: true,
    writable: true,
});

//! deno_url

import * as url from "ext:deno_url/00_url.js";
import * as urlPattern from "ext:deno_url/01_urlpattern.js";

Object.defineProperty(globalThis, "URL", {
    value: url.URL,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "URLPattern", {
    value: url.URLPattern,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "URLSearchParams", {
    value: url.URLSearchParams,
    configurable: true,
    writable: true,
});

//! deno_web

import * as infra from "ext:deno_web/00_infra.js";
import * as DOMException from "ext:deno_web/01_dom_exception.js";
import * as mimesniff from "ext:deno_web/01_mimesniff.js";
import * as event from "ext:deno_web/02_event.js";
import * as structuredClone from "ext:deno_web/02_structured_clone.js";
import * as timers from "ext:deno_web/02_timers.js";
import * as abortSignal from "ext:deno_web/03_abort_signal.js";
import * as globalInterfaces from "ext:deno_web/04_global_interfaces.js";
import * as base64 from "ext:deno_web/05_base64.js";
import * as streams from "ext:deno_web/06_streams.js";
import * as encoding from "ext:deno_web/08_text_encoding.js";
import * as file from "ext:deno_web/09_file.js";
import * as fileReader from "ext:deno_web/10_filereader.js";
import * as location from "ext:deno_web/12_location.js";
import * as messagePort from "ext:deno_web/13_message_port.js";
import * as compression from "ext:deno_web/14_compression.js";
import * as performance from "ext:deno_web/15_performance.js";
import * as imageData from "ext:deno_web/16_image_data.js";

Object.defineProperty(globalThis, "AbortController", {
    value: abortSignal.AbortController,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "AbortSignal", {
    value: abortSignal.AbortSignal,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "Blob", {
    value: file.Blob,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "ByteLengthQueuingStrategy", {
    value: streams.ByteLengthQueuingStrategy,
});

Object.defineProperty(globalThis, "CloseEvent", {
    value: event.CloseEvent,
});

Object.defineProperty(globalThis, "CompressionStream", {
    value: compression.CompressionStream,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "CountQueuingStrategy", {
    value: streams.CountQueuingStrategy,
});

Object.defineProperty(globalThis, "CustomEvent", {
    value: event.CustomEvent,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "DecompressionStream", {
    value: compression.DecompressionStream,
    configurable: true,
    writable: true,
});


Object.defineProperty(globalThis, "DOMException", {
    value: DOMException,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "ErrorEvent", {
    value: event.ErrorEvent,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "Event", {
    value: event.Event,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "EventTarget", {
    value: event.EventTarget,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "File", {
    value: file.File,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "FileReader", {
    value: fileReader.FileReader,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "MessageEvent", {
    value: event.MessageEvent,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "Performance", {
    value: performance.Performance,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "PerformanceEntry", {
    value: performance.PerformanceEntry,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "PerformanceMark", {
    value: performance.PerformanceMark,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "PerformanceMeasure", {
    value: performance.PerformanceMeasure,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "PromiseRejectionEvent", {
    value: event.PromiseRejectionEvent,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "ProgressEvent", {
    value: event.ProgressEvent,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "ReadableStream", {
    value: streams.ReadableStream,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "ReadableStreamDefaultReader", {
    value: streams.ReadableStreamDefaultReader,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "TextDecoder", {
    value: encoding.TextDecoder,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "TextEncoder", {
    value: encoding.TextEncoder,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "TextDecoderStream", {
    value: encoding.TextDecoderStream,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "TextEncoderStream", {
    value: encoding.TextEncoderStream,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "TransformStream", {
    value: streams.TransformStream,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "MessageChannel", {
    value: messagePort.MessageChannel,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "MessagePort", {
    value: messagePort.MessagePort,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "WritableStream", {
    value: streams.WritableStream,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "WritableStreamDefaultWriter", {
    value: streams.WritableStreamDefaultWriter,
});

Object.defineProperty(globalThis, "WritableStreamDefaultController", {
    value: streams.WritableStreamDefaultController,
});

Object.defineProperty(globalThis, "ReadableByteStreamController", {
    value: streams.ReadableByteStreamController,
});

Object.defineProperty(globalThis, "ReadableStreamBYOBReader", {
    value: streams.ReadableStreamBYOBReader,
});

Object.defineProperty(globalThis, "ReadableStreamBYOBRequest", {
    value: streams.ReadableStreamBYOBRequest,
});

Object.defineProperty(globalThis, "ReadableStreamDefaultController", {
    value: streams.ReadableStreamDefaultController,
});

Object.defineProperty(globalThis, "TransformStreamDefaultController", {
    value: streams.TransformStreamDefaultController,
});

Object.defineProperty(globalThis, "ImageData", {
    value: imageData.ImageData,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "atob", {
    value: base64.atob,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "btoa", {
    value: base64.btoa,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "clearInterval", {
    value: timers.clearInterval,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "clearTimeout", {
    value: timers.clearTimeout,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "performance", {
    value: performance.performance,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "reportError", {
    value: event.reportError,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "setInterval", {
    value: timers.setInterval,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "setTimeout", {
    value: timers.setTimeout,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "structuredClone", {
    value: messagePort.structuredClone,
    enumerable: true,
    configurable: true,
    writable: true,
});

//! deno_net

import * as net from "ext:deno_net/01_net.js";
import * as tls from "ext:deno_net/02_tls.js";

//! deno_fetch

import * as headers from "ext:deno_fetch/20_headers.js";
import * as formData from "ext:deno_fetch/21_formdata.js";
import * as request from "ext:deno_fetch/23_request.js";
import * as response from "ext:deno_fetch/23_response.js";
import * as fetch from "ext:deno_fetch/26_fetch.js";
import * as eventSource from "ext:deno_fetch/27_eventsource.js";

// Set up the callback for Wasm streaming ops
Deno.core.setWasmStreamingCallback(fetch.handleWasmStreaming);

Object.defineProperty(globalThis, "fetch", {
    value: fetch.fetch,
    enumerable: true,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "Request", {
    value: request.Request,
    enumerable: false,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "Response", {
    value: response.Response,
    enumerable: false,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "Headers", {
    value: headers.Headers,
    enumerable: false,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "FormData", {
    value: formData.FormData,
    enumerable: false,
    configurable: true,
    writable: true,
});

//! deno_crypto

import * as crypto from "ext:deno_crypto/00_crypto.js";

Object.defineProperty(globalThis, "CryptoKey", {
    value: crypto.CryptoKey,
    enumerable: false,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "crypto", {
    value: crypto.crypto,
    enumerable: false,
    configurable: true,
    writable: false,
});

Object.defineProperty(globalThis, "Crypto", {
    value: crypto.Crypto,
    enumerable: false,
    configurable: true,
    writable: true,
});

Object.defineProperty(globalThis, "SubtleCrypto", {
    value: crypto.SubtleCrypto,
    enumerable: false,
    configurable: true,
    writable: true,
});