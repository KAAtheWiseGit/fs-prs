use crate::object::Type;
use std::{error, fmt, io};

use camino::{FromPathBufError, FromPathError, Utf8PathBuf};

#[derive(Debug)]
pub enum ValidationError {
	NotExists,
	WrongType { expected: Type, got: Type },
	NotUTF8,
}

impl fmt::Display for ValidationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::NotExists => write!(f, "object no longer exists"),
			Self::WrongType { expected, got } => write!(
				f,
				"expected the object to be {}, got {}",
				expected, got
			),
			Self::NotUTF8 => {
				write!(f, "the path is not a valid UTF-8")
			}
		}
	}
}

#[derive(Debug)]
pub enum Error {
	IO(io::Error),
	Validation(ValidationError),
	MoveDestNotDir,
	NameTaken(Utf8PathBuf),
	CannotRenameRoot,
	ForbiddenNameChars,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::IO(error) => write!(f, "IO error: {error}"),
			Self::Validation(error) => {
				write!(f, "validation error: {error}")
			}
			Self::MoveDestNotDir => {
				write!(f, "move destination is not a directory")
			}
			Self::NameTaken(path) => write!(
				f,
				"a file object at {} already exists",
				path
			),
			Self::CannotRenameRoot => {
				write!(f, "root directory can't be renamed")
			}
			Self::ForbiddenNameChars => write!(
				f,
				"file names can't include '/' or '\\0'"
			),
		}
	}
}

impl error::Error for Error {}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Self::IO(value)
	}
}

impl From<FromPathBufError> for Error {
	fn from(_value: FromPathBufError) -> Self {
		Self::Validation(ValidationError::NotExists)
	}
}

impl From<FromPathError> for Error {
	fn from(_value: FromPathError) -> Self {
		Self::Validation(ValidationError::NotExists)
	}
}
