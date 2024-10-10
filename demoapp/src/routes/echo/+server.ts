import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
    const body = await request.body;
    if (!body) return new Response();

    const reader = body.getReader();
    let stream = new ReadableStream({
        start(controller) {
            function push() {
                reader.read().then(({ done, value }) => {
                    if (done) {
                        controller.close();
                        return;
                    }
                    controller.enqueue(value);
                    push();
                });
            }

            push();
        },
    });

    return new Response(stream, {
        headers: {
            'content-type': 'application/octet-stream',
        }
    });
};

