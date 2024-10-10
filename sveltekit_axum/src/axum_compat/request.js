import {
    readableStreamForRid,
} from "ext:deno_web/06_streams.js";
import { InnerBody } from "ext:deno_fetch/22_body.js";
import {
    fromInnerRequest,
    newInnerRequest,
} from "ext:deno_fetch/23_request.js";

/**
 * @param {[number, string, string, [string, string][]]} requestRep
 * @returns {Request}
 */
export function requestFromRep(requestRep) {
    const { 0: readStreamRid, 1: method, 2: url, 3: headers } = requestRep;

    /** @type {ReadableStream<Uint8Array> | undefined} */
    let body = null;
    // There might be a body, but we don't expose it for GET/HEAD requests.
    // It will be closed automatically once the request has been handled and
    // the response has been sent.
    if (method !== "GET" && method !== "HEAD") {
        body = readableStreamForRid(readStreamRid, false);
    }

    const innerRequest = newInnerRequest(
        method,
        url,
        () => headers,
        body !== null ? new InnerBody(body) : null,
        false,
    );
    const request = fromInnerRequest(
        innerRequest,
        "immutable",
        false,
    );

    return request;
}