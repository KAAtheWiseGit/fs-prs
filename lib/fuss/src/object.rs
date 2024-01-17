// The module is currently vunerable to TOCTOU.

use std::{
	env, fmt,
	fs::{self, DirBuilder, File, FileType},
	path::Path,
};

use camino::{Utf8Path, Utf8PathBuf};
use sha2::{Digest, Sha256};

use super::error::{Error, ValidationError};
use super::utils::hash_dir;
use super::Result;

mod mode;

// XXX: `Other` variant.  Probably a unit type, but there are also sockets and
// such.  Doesn't make much sense for the current command makeup, but if I ever
// add an `open` command, might become an issue.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
	File,
	Dir,
	Symlink,
}

impl From<FileType> for Type {
	fn from(value: FileType) -> Self {
		match value {
			_ if value.is_file() => Self::File,
			_ if value.is_dir() => Self::Dir,
			_ if value.is_symlink() => Self::Symlink,
			// needs special handling or proof of impossibility
			_ => unimplemented!(),
		}
	}
}

// XXX: paths?
impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::File => write!(f, "file"),
			Self::Dir => write!(f, "directory"),
			Self::Symlink => write!(f, "symbolic link"),
		}
	}
}

// XXX: Needs an owned version.  On Linux this means file descriptors and sanity
// checks, on Windows there is some kind of API for it.
pub struct Object {
	// INVARIANT: always an absolute path
	path: Utf8PathBuf,
	r#type: Type,
}

// TODO: figure out interobject relationship.  Movements and such should
// probably be delegated to an owned version.
impl Object {
	/// Open an existing FS object.
	pub fn from_existing(path: &Path) -> Result<Self> {
		let path = to_absolute(path.try_into()?)?;

		let r#type = match fs::metadata(&path)?.file_type() {
			f if f.is_file() => Type::File,
			f if f.is_dir() => Type::Dir,
			f if f.is_symlink() => Type::Symlink,
			_ => todo!(),
		};

		Ok(Object { r#type, path })
	}

	/// Create a file at `path`.
	pub fn file(path: &Path) -> Result<Self> {
		let path = to_absolute(path.try_into()?)?;

		if let Some(parent) = path.parent() {
			DirBuilder::new().recursive(true).create(parent)?;
		}

		File::create(&path)?;
		Ok(Object {
			r#type: Type::File,
			path,
		})
	}

	/// Create a directory at `path`.
	pub fn dir(path: &Path) -> Result<Self> {
		let path = to_absolute(path.try_into()?)?;

		DirBuilder::new().recursive(true).create(&path)?;
		Ok(Object {
			r#type: Type::Dir,
			path,
		})
	}

	/// Move the object to a `to` directory.  `to` must not have an existing
	/// directory/file with the same name as the moved object.
	pub fn r#move(&mut self, to: &Path) -> Result<()> {
		self.validate()?;

		let to = to_absolute(to.try_into()?)?;

		if !to.is_dir() {
			return Err(Error::MoveDestNotDir);
		}

		// valid UTF-8, because both to and name were checked
		let dest = to.join(self.name());
		not_exist(&dest)?;

		fs::rename(&self.path, dest)?;
		Ok(())
	}

	/// Change the actual name (not the path) of the object.
	///
	/// # Notes
	///
	/// Calling this on a directory object invalidates objects referencing
	/// files inside it.
	pub fn rename(&mut self, to: String) -> Result<()> {
		self.validate()?;

		if to.contains(['/', '\0']) {
			return Err(Error::ForbiddenNameChars);
		}

		let parent = match self.path.parent() {
			Some(p) => p,
			None => return Err(Error::CannotRenameRoot),
		};

		let dest = parent.join(to);
		not_exist(&dest)?;

		fs::rename(&self.path, dest)?;
		Ok(())
	}

	pub fn delete(&self) -> Result<()> {
		self.validate()?;

		match &self.r#type {
			Type::File => {
				// This can't throw is a dir error and file
				// doesn't exist errors due to validation.  The
				// only possible one is permission error.
				fs::remove_file(&self.path)?;
			}
			Type::Dir => {
				fs::remove_dir_all(&self.path)?;
			}
			Type::Symlink => unimplemented!(),
		}

		Ok(())
	}

	/// Do miscellaneous sanity checks, which verify that the object
	/// reference is still valid.
	fn validate(&self) -> Result<()> {
		// path exists
		if !self.path.exists() {
			return Err(Error::Validation(
				ValidationError::NotExists,
			));
		}

		// type matches
		let got = fs::metadata(&self.path)?.file_type().into();
		let expected = self.r#type;
		if expected != got {
			return Err(Error::Validation(
				ValidationError::WrongType { expected, got },
			));
		}

		Ok(())
	}

	fn name(&self) -> &str {
		self.path
			.file_name()
			// Unwrap file name or empty string for root `/`
			// Can't have other cases, because the path is absolute.
			.unwrap_or_default()
	}

	pub fn std_path(&self) -> &Path {
		self.path.as_std_path()
	}

	pub fn sha256(&self) -> Result<[u8; 32]> {
		let mut hasher = Sha256::new();

		match self.r#type {
			Type::File => hasher.update(fs::read(&self.path)?),
			Type::Dir => hash_dir(self.std_path(), &mut hasher)?,
			_ => (),
		}

		Ok(hasher.finalize().into())
	}
}

/// Convert a relative path to absolute.  Returns owned version of absolute
/// paths.
///
/// # Note
///
/// Clones the path.
fn to_absolute(path: &Utf8Path) -> Result<Utf8PathBuf> {
	Ok(if path.is_relative() {
		let current_dir: Utf8PathBuf =
			env::current_dir()?.try_into()?;
		current_dir.join(path)
	} else {
		path.to_owned()
	})
}

/// Check that the destination doesn't exist
fn not_exist(dest: &Utf8Path) -> Result<()> {
	if dest.exists() {
		Err(Error::NameTaken(dest.to_owned()))
	} else {
		Ok(())
	}
}
