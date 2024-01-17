#![allow(unused)]

pub trait Construct {
	/// Wherever the object depends on another in the history tree.
	fn depends_on(&self, other: &Self) -> bool;

	/// Scan a slice
	// XXX: use an iterator instead, avoids Sized bound
	fn scan(&self, others: &[Self]) -> Vec<bool>
	where
		Self: Sized,
	{
		let mut out = Vec::new();
		for other in others {
			out.push(self.depends_on(other));
		}
		out
	}
}
