import { Server } from './index.js';
import { manifest } from './manifest.js';

const server = new Server(manifest);
await server.init({
	env: {
		/* TODO */
	}
});

console.log("Init done");

/**
 * @param {[number, string, string, [string, string][]]} requestRep
 * @returns {Promise<Response>}
 */
async function handler(requestRep) {
	/** @type {Request} */
	const request = AxumCompat.requestFromRep(requestRep);

	console.log(`[Js handler] processing request for '${request.url}'`);

	const response = server.respond(request, {
		getClientAddress() {
			return 'localhost';
		}
	});

	return response;
}

export { handler };
