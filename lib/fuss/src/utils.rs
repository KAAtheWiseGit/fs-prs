#![allow(unused)]

use std::{fs, path::Path};

use sha2::Digest;

use super::Result;

/// Hash all files in a directory into a hasher.
// XXX: this might give the same hash for same contents under different paths,
// investigate another ways to checksum a directory.
pub fn hash_dir(dir: &Path, hasher: &mut impl Digest) -> Result<()> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				hash_dir(&path, hasher);
			} else {
				hasher.update(fs::read(&path)?);
			}
		}
	}

	Ok(())
}
