use snowboard::{response, Request, Response, Url};

use crate::info;

pub async fn handle(request: &Request, url: &Url<'_>) -> Response {
	info!("Got API request {:#?}", request);

	// do your magic here

	response!(ok, format!("Hello from `/api`! You requested to `{}`", url))
}
