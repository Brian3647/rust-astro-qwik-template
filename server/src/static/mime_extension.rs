use std::io;

macro_rules! generate_mime {
	($($extension:literal => $content_type:literal)*) => {
		lazy_static::lazy_static! {
			pub static ref MIME_EXTENSIONS: std::collections::HashMap<&'static str, &'static str> = {
				let mut map = std::collections::HashMap::new();
				$(
					map.insert($extension, $content_type);
				)*
				map
			};
		}
	};
}

generate_mime!(
	"html" => "text/html"
	"css" => "text/css"
	"js" => "text/javascript"
	"json" => "application/json"
	"svg" => "image/svg+xml"
	// TODO: Add more MIME types as needed
);

/// Get the MIME type for the given file extension, returning an `io` error if
/// it isn't in the map.
pub fn get_from_mime(ext: &str) -> io::Result<&'static str> {
	MIME_EXTENSIONS.get(ext).copied().ok_or_else(|| {
		io::Error::new(
			io::ErrorKind::InvalidInput,
			format!("Unknown file extension: {}", ext),
		)
	})
}
