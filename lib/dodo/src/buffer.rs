#![allow(unused)]

// XXX: investigate ways to control behaviour:
//
// - integrating associated constants with functions to handle the buffer.  It
// is tricky because they need to depend on `T`.
//
// - using `PhantomData`.  Inelegant and requires runtime creation.
//
// - Generic parameter.  Unclear how to fetch it in generic `impl` code.
//
// - Traits.  The way to go for now, since it allows performance tweaks.

struct HistoryBuffer<T> {
	stack: Vec<T>,
}

impl<T> HistoryBuffer<T> {
	/// Push a command at the top of the buffer.
	fn push(&mut self, value: T) {
		self.stack.push(value);
	}

	/// Revert the last command.
	fn revert(&mut self) {
		// This needs to build a dependency structure, see the algorithm
		// docs on scanning
		unimplemented!()
	}

	// XXX methods:
	//
	// - revert an arbitrary construct
}
