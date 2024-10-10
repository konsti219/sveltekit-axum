import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
    // buffer request body
    let body = new Uint8Array(await request.arrayBuffer());

    let interval: number | null = null;
    let stream = new ReadableStream({
        start(controller) {
            let i = 0;
            interval = setInterval(() => {
                if (i >= body.byteLength) {
                    if (interval !== null) {
                        clearInterval(interval);
                        interval = null;
                    }

                    controller.close();
                    return;
                }

                controller.enqueue(new Uint8Array([body[i]]));
                i++;
            }, 500);
        },
        cancel(_reason) {
            if (interval !== null) {
                clearInterval(interval);
                interval = null;
            }
        }
    });

    return new Response(stream, {
        headers: {
            // 'content-type': 'application/octet-stream',
            'content-type': 'text/plain',
        }
    });
};

