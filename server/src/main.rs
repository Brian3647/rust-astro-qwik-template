use std::{env, process};

use snowboard::{Request, Response, Server};

mod api;
mod log;
mod r#static;

use r#static::StaticFileHandler;

lazy_static::lazy_static! {
	static ref STATIC_FILES: StaticFileHandler = StaticFileHandler::expect_init();
}

const DEFAULT_PORT: u16 = 8080;

fn main() {
	let mut argv = env::args();

	let port = if let Some(arg) = argv.nth(1).and_then(|x| x.parse::<u16>().ok()) {
		arg
	} else if let Some(env_port) = env::var("PORT").ok().and_then(|x| x.parse::<u16>().ok()) {
		env_port
	} else {
		warn!("No port provided and env `PORT` is not set, using default ({DEFAULT_PORT})");
		DEFAULT_PORT
	};

	let addr = format!("0.0.0.0:{}", port);

	// Initialize the static files handler by making sure
	// `Defer` is called before `STATIC_FILES` is used.
	_ = STATIC_FILES.root();

	info!("Starting server at http://{}", addr);

	Server::new(addr)
		.unwrap_or_else(|err| {
			error!("Failed to start server: {}", err);
			process::exit(1);
		})
		.with_default_headers()
		.run(handler)
}

fn handler(request: Request) -> Response {
	let url = request.parse_url();

	match url.at(0) {
		Some("api") => api::handle(&request, url),
		_ => STATIC_FILES.get_response_or_404(&url.path.join("/")),
	}
}
