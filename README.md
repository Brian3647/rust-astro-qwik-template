# Fullstack application template

This is a big monorepo that contains a fullstack application template. It is divided in 2 folders:

-   `server`: The backend code ([more info](#server))
-   `web`: The frontend code ([more info](#web))

And additionally includes `justfile`, `.editorconfig`, `.gitignore`, a basic `.github` folder with a workflow.

`justfile` is meant to be used with [just](https://github.com/casey/just) to manage the project (see the list of commands [here](#justfile-building-running-etc)).

The server and frontend are not dependant on eachother, so they can be swapped easily.

## What this template has

A working static file & API server with a simple logging system, a working astro + qwik frontend with an index and a 404 page, a `.github` folder with a workflow that builds, tests and checks the code, a `justfile` with everything you would need to manage the monorepo, prettier and rustfmt configurations, editorconfig files and other set-up configuration files.

## What this template uses

-   [rust](https://www.rust-lang.org/)
-   [snowboard](https://github.com/Brian3647/snowboard)
-   [just](https://github.com/casey/just)
-   [bun.js](https://bun.sh/)
-   [astro](https://astro.build/)
-   [qwik](https://qwik.builder.io/docs/)

## Server

The server is a rust application that uses [snowboard](https://github.com/Brian3647/snowboard) to host an extremely fast server. It is divided in 2 folders inside `src`. When running, you can pass a port as an argument or set the `PORT` environment variable.
If neither is set, it defaults to `8080`.

### `static`

All the logic that handles static file serving. By default, it does not have much MIME type recognition, but you can easily add your own by adding them in `server/src/static/mime_extension.rs`:

```rs
// ...
generate_mime!(
	...
	"your custom extension" => "any mime type"
);
// ...
```

By default, the server hosts the `web/dist` folder, but you can change that by changing the `DIST` constant in `server/src/static/mod.rs`.

### `api`

The api folder contains all the logic you want it to contain. This template only gives you `mod.rs`, which exports the entry point of the api. You can add more paths using `snowboard`'s [routing](https://github.com/Brian3647/snowboard#routing). For conviniency, you don't need to run `parse_url` since `handler` already takes an `Url` struct as an argument.

## Web

The web has a simpler set-up. It's an [astro](https://astro.build/) application that uses [qwik](https://qwik.builder.io/docs/) to maximize performance. You can add static files to `public`. To add new pages, you can just create a new `.astro` file inside `src` (in the template, `404.astro` and `index.astro` are provided). The folders `components` and `layouts` are also provided, but you can remove them or use them in any other structure you want (as long as astro accepts it).

## Justfile (building, running, etc.)

The commands are:

-   `just build`: Builds everything
-   `just run`: Builds everything & runs the server with `--release`
-   `just run-dev` Builds everything & Runs the server without `--release`
-   `just test`: Runs the tests
-   `just check`: Checks the code
-   `just fmt`: Formats the code

Every command except `run` and `run-dev` also has a `-web` or `-server` variant, which only runs the command for the web or server respectively. (eg `just build-web` or `just fmt-server`)

## License

Everything is licensed under either the [MIT license](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE).
