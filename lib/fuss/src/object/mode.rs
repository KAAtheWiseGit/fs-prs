use std::{fmt, iter::zip};

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Owner {
	User = 64,
	Group = 8,
	Other = 1,
}

const OWNERS: &[Owner; 3] = &[Owner::User, Owner::Group, Owner::Other];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Permission {
	Read = 4,
	Write = 2,
	Execute = 1,
}

const PERMISSIONS: &[Permission; 3] =
	&[Permission::Read, Permission::Write, Permission::Execute];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Mode(u32);

impl Mode {
	pub fn from_rwx_str(s: &str) -> Option<Self> {
		if s.len() != 9 {
			return None;
		}
		for (ch, expected_ch) in
			zip(s.chars(), ['r', 'w', 'x'].repeat(3))
		{
			if ch != expected_ch && ch != '-' {
				return None;
			}
		}

		let mut out = Self(0);

		// A cartesian product of all (owner, perm) pairs.
		let prod = OWNERS.iter().cartesian_product(PERMISSIONS.iter());
		for (ch, (owner, perm)) in zip(s.chars(), prod) {
			if ch != '-' {
				out.add_permission(*owner, *perm);
			}
		}

		Some(out)
	}

	#[allow(clippy::arithmetic_side_effects)]
	// [`Owner`] conversion is guaranteed to be 64 or less, while
	// [`Permission`] will be less than or equal to 4.
	fn add_permission(&mut self, owner: Owner, permission: Permission) {
		self.0 += owner as u32 * permission as u32;
	}

	#[allow(clippy::arithmetic_side_effects)]
	// Same as [`add_permission`].
	fn remove_permission(&mut self, owner: Owner, permission: Permission) {
		self.0 -= owner as u32 * permission as u32;
	}

	#[allow(clippy::arithmetic_side_effects)]
	// u32 represenation of owner is never zero
	fn get_owner_perms(&self, owner: Owner) -> u32 {
		self.0 / owner as u32
	}

	fn perms_str(&self) -> String {
		OWNERS.iter()
			.map(|owner| u32_to_rwx(self.get_owner_perms(*owner)))
			.collect::<Vec<_>>()
			.join("")
	}
}

impl fmt::Display for Mode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.perms_str())
	}
}

impl std::str::FromStr for Mode {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::from_rwx_str(s).ok_or(())
	}
}

fn u32_to_rwx(value: u32) -> &'static str {
	match value % 8 {
		0 => "---",
		1 => "--x",
		2 => "-w-",
		3 => "-wx",
		4 => "r--",
		5 => "r-x",
		6 => "rw-",
		7 => "rwx",
		_ => unreachable!(),
	}
}

#[cfg(test)]
mod test {
	use super::{Mode, Owner, Permission};

	#[test]
	fn perms() {
		let mut mode = Mode(0);
		mode.add_permission(Owner::User, Permission::Read);
		mode.add_permission(Owner::User, Permission::Execute);
		mode.add_permission(Owner::Group, Permission::Read);
		mode.add_permission(Owner::Other, Permission::Write);

		assert_eq!(mode.0, 0o542);
	}

	const PAIRS: [(u32, &'static str); 4] = [
		(0o755, "rwxr-xr-x"),
		(0o636, "rw--wxrw-"),
		(0o542, "r-xr---w-"),
		(0o000, "---------"),
	];

	#[test]
	fn perms_str() {
		for (mode, s) in PAIRS {
			assert_eq!(s, Mode(mode).perms_str());
		}
	}

	#[test]
	fn parse() {
		for (mode, s) in PAIRS {
			assert_eq!(s.parse(), Ok(Mode(mode)));
		}
	}
}
