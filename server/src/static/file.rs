use std::{fs, io, path::PathBuf};

use snowboard::{headers, ResponseLike};

use super::mime_extension::get_from_mime;

pub struct File {
	pub contents: Vec<u8>,
	pub content_type: String,
}

impl TryFrom<PathBuf> for File {
	type Error = io::Error;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		let contents = fs::read(&path)?;
		let content_type = match path.extension().and_then(|x| x.to_str()) {
			Some(ext) => get_from_mime(ext)?,
			None => "application/octet-stream",
		}
		.into();
		Ok(Self {
			contents,
			content_type,
		})
	}
}

impl ResponseLike for File {
	fn to_response(self) -> snowboard::Response {
		snowboard::response!(
			ok,
			self.contents,
			headers! { "Content-Type" => self.content_type }
		)
	}
}

impl ResponseLike for &File {
	fn to_response(self) -> snowboard::Response {
		snowboard::response!(
			ok,
			*self.contents,
			headers! { "Content-Type" => self.content_type }
		)
	}
}
