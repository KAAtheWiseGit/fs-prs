use clap::{builder, Arg, ArgAction, Command};

/// Return an arg, processing a path.
// XXX: add wildcards processing
fn path(name: &'static str, help: &'static str) -> Arg {
	Arg::new(name)
		.value_parser(builder::PathBufValueParser::new())
		.required(true)
		.help(help)
}

fn delete() -> Command {
	Command::new("delete")
		.about("Delete an object")
		.long_about("Delete files, directories (recursively), or other filesystem objects.  All deleted objects can be restored from history.")
		.arg(path("path", "Path to target file"))
}

fn copy() -> Command {
	Command::new("copy")
		.about("Copy an object to a directory")
		.arg(path("src", "Object to be copied"))
		.arg(path("dst", "Directory to copy to"))
}

fn r#move() -> Command {
	Command::new("move")
		.about("Move an object to a directory")
		.arg(path("src", "Object to be moved"))
		.arg(path("dst", "Directory to move to"))
}

pub fn command() -> Command {
	Command::new("fs")
		.about("Filesystem utility with undo")
		.long_version("v0.1.0")
		.subcommand(delete())
		.subcommand(copy())
		.subcommand(r#move())
		.arg_required_else_help(true)
		.arg(Arg::new("force")
			.short('f')
			.long("force")
			.action(ArgAction::SetTrue)
			.help("Overwrite destination files and mute warnings")
			.global(true))
		.arg(Arg::new("verbose")
			.short('v')
			.long("verbose")
			.action(ArgAction::SetTrue)
			.help("Print informational messages")
			.global(true))
}
