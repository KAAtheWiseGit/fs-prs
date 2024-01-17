#![allow(unused)]

use std::path::{Path, PathBuf};

use uuid::{NoContext, Timestamp, Uuid};

use fuss::{Object, Result};

pub enum Type {
	Move,
	Copy,
	Delete,
}

pub struct Command {
	// Unique UUIDv7 id
	id: Uuid,
	// Command type
	r#type: Type,
	// The object which was the target
	src: Object,
	// Hash derived from `src`
	hash: [u8; 32],
	// Where the object ended up, if applicable
	dst: Option<PathBuf>,
}

impl Command {
	pub fn new(
		r#type: Type,
		src: Object,
		dst: Option<&Path>,
	) -> Result<Command> {
		let id = Uuid::new_v7(Timestamp::now(NoContext));
		let hash = src.sha256()?;
		let dst = dst.map(|dst| dst.to_path_buf());

		Ok(Command {
			id,
			r#type,
			src,
			hash,
			dst,
		})
	}

	/// Executes the command.
	///
	/// # Invariants
	///
	/// - Must only be called once.
	pub fn execute(&self) {
		match self.r#type {
			Type::Delete => self.delete(),
			Type::Move => {}
			Type::Copy => {}
		};
	}

	/// Revert a command back from store.
	pub fn revert(&self) {
		todo!()
	}

	fn delete(&self) {
		// TODO: copy the object to store
		self.src.delete();
	}

	fn r#move(&mut self) {
		// (optional) copy the object to store
		// move the object to destination
		#[allow(clippy::unwrap_used)]
		self.src.r#move(&self.dst.clone().unwrap()).unwrap();

		// XXX: it has to be decided wherever `fs` copies the moved
		// object.  Given the spatial constraints, it doesn't make much
		// sense to do it.  Thus, the undo of a move probably means
		// moving the edited content back to where it was.
	}

	fn copy(&self) {
		// (optional) copy the object to store
		// copy the object to destination

		// XXX: the same logic as with `move` doesn't quite work here,
		// since the states of objects might diverge.  The most
		// straightforward thing to do is to just delete the copied
		// object.  But this means undo/redo will sometimes change the
		// files.  So, it should either be blocked on diverging, or the
		// redo functionality needs to be abandoned.
	}
}

// XXX: a builder would be nice, but I want static verification.
// See https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Static.20verification.20of.20builder.20patterns
// There are some options, but they are kind of hacky.
// struct CommandBuilder {}
