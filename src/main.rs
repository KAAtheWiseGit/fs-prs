#![allow(unused)]

mod cli;
mod command;

use std::path::PathBuf;

use camino::Utf8Path;
use clap::ArgMatches;

use command::{Command, Type};
use fuss::{Object, Result};

// # Algorithm
//
// - Clap processes the command.
// - Fetch the command type.
// - Fetch and validate the arguments:
//
//   - All paths are UTF-8.
//   - Object can be created.
//   - Destination is valid.
//   - Command can be executed.
//
// - Raise interactive hints if not `force`.
// - Execute the command.
// - Push the command onto the `dodo` stack.
fn main() {
	if let Err(err) = run() {
		eprintln!("{err}")
	}
}

fn run() -> Result<()> {
	let matches = cli::command().get_matches();

	let r#type = match matches.subcommand_name() {
		Some("delete") => Type::Delete,
		Some("move") => Type::Move,
		Some("copy") => Type::Copy,
		// TODO: verify this is impossible
		_ => unreachable!(),
	};
	let matches = get_subcommand_matches(&matches);
	let src = get_subcommand_source(matches, &r#type);
	let dst = get_subcommand_destination(matches, &r#type)
		.map(|path| path.as_path());
	let object = Object::from_existing(src)?;

	Command::new(r#type, object, dst)?.execute();

	Ok(())
}

#[allow(clippy::unwrap_used)]
fn get_subcommand_matches(matches: &ArgMatches) -> &ArgMatches {
	matches.subcommand_matches(
		matches.subcommand_name()
			// if no subcommand is give `fs` terminates with help.
			// If a wrong subcommand is given, `clap` catches the
			// error.  Thus, `None` here is not possible
			.unwrap(),
	)
	// Likewise, we have already verified that the subcommand is present
	.unwrap()
}

#[allow(clippy::unwrap_used)]
fn get_subcommand_source<'a>(
	matches: &'a ArgMatches,
	r#type: &Type,
) -> &'a PathBuf {
	// These arguments are required, thus they are always present
	match r#type {
		Type::Delete => matches.get_one("path").unwrap(),
		Type::Move => matches.get_one("src").unwrap(),
		Type::Copy => matches.get_one("src").unwrap(),
	}
}

#[allow(clippy::unwrap_used)]
fn get_subcommand_destination<'a>(
	matches: &'a ArgMatches,
	r#type: &Type,
) -> Option<&'a PathBuf> {
	// These arguments are required, thus they are always present
	match r#type {
		Type::Delete => None,
		// This is needed to assert that the variables are present, as
		// it's presumed they are present for those command types.
		Type::Move => Some(matches.get_one("dst").unwrap()),
		Type::Copy => Some(matches.get_one("dst").unwrap()),
	}
}
