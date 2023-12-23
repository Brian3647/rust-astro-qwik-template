//! Internal utilities

use crate::error;
use std::{
	fs, io,
	path::{Path, PathBuf},
};

/// Logs an error and returns an [`io::Error`] saying a path contains invalid UTF-8.
pub fn invalid_utf8_detected<T>(path: T) -> io::Error
where
	T: AsRef<Path>,
{
	let error_msg = format!("Path '{}' contains invalid UTF-8", path.as_ref().display());
	error!("{}", error_msg);
	io::Error::new(io::ErrorKind::InvalidData, error_msg)
}

/// Gets all the paths in a directory, including subdirectories.
pub fn get_paths<T>(dir: T) -> io::Result<Vec<PathBuf>>
where
	T: AsRef<Path>,
{
	let mut result = vec![];

	for entry in fs::read_dir(dir)? {
		let path = entry?.path();

		if !path.is_dir() {
			result.push(path);
			continue;
		}

		let other = get_paths(path)?;

		result.extend(other);
	}

	Ok(result)
}
