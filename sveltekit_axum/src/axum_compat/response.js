import {
    writableStreamForRid,
} from "ext:deno_web/06_streams.js";
import {
    toInnerResponse,
} from "ext:deno_fetch/23_response.js";

/**
 * @param {Response} response
 * @param {number} writeStreamRid
 * @returns {[number, [string, string][]]}
 */
export function responseToRep(response, writeStreamRid) {
    const body = response.body;
    const { status, headerList, /*body*/ } = toInnerResponse(response);

    let writeStream = writableStreamForRid(writeStreamRid);
    //! ReadableStream.pipeTo returns a Promise, which we are intentionally leaking to have it run in the background
    //! instead of blocking the fucntion return.
    if (body) {
        body.pipeTo(writeStream);
    } else {
        ReadableStream.from([]).pipeTo(writeStream);
    }

    return [status, headerList];
}