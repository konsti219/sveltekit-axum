# sveltekit-axum

A highly experimental webframework that renders your SvelteKit frontend inside an Axum route using an embedded Deno instance.

## Getting started

Take a look at the `demoapp` directory for how an application might be structured. You can run the project using `nix run .#demoapp-bin`.

## Features

Working:

- Rendering SvelteKit pages and API routes
- Processing multiple requests at once
- Serving static files via optimzed Axum routes
- Request and Response streaming
- Fast startup and rendering thanks to slimmed down Deno Runtime

Not yet implemented:

- Calling Axum routes from SvelteKit routes during server side rendering
- Better dev setup (hot reload, etc.)
- Configurable Deno Runtime to include more Js/Web features
