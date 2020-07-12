# cstatus

## Requirements

NodeJs - [Install](https://nodejs.org/en/download/)
Rust - [Install](https://www.rust-lang.org/tools/install)

## Get started with Development

Start Rust server and [Rollup](https://rollupjs.org) in watch mode

```bash
yarn dev
```

Navigate to [localhost:3000](http://localhost:3000). You should see your app running.
Navigate to [localhost:3000/graphql](http://localhost:3000/graphql). To get to the graphql playground.
All svelte component live in `client` directory. Save any changes live-reloading.
All static files are served from `public` direcotry. Including the JS code compiled by Svelte Compiler.

## Building and running in production mode

To create an optimised version of the app:

```bash
npm run build
cargo build
```

## Built With

[Rust](https://www.rust-lang.org/)

[Svelte](https://svelte.dev/)
