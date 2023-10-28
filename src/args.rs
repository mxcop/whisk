use std::path::PathBuf;

use clap::{arg, Command};

/// Mix CLI clap command.
pub fn cli() -> Command {
    Command::new("whisk")
        .about("A simplistic build system for C/C++")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Creates a new project")
                .arg(arg!(name: <NAME> "New project name").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("build")
                .about("Builds a project")
                .arg(arg!(path: <PATH> "Project path").required(false)
                    .value_parser(clap::value_parser!(PathBuf)).default_value("."))
        )
        .subcommand(
            Command::new("run")
                .about("Builds & runs a project")
                .arg(arg!(path: <PATH> "Project path").required(false)
                    .value_parser(clap::value_parser!(PathBuf)).default_value("."))
        )
        .subcommand(
            Command::new("clean")
                .about("Removes the bin directory of a project")
                .arg(arg!(path: <PATH> "Project path").required(false)
                    .value_parser(clap::value_parser!(PathBuf)).default_value("."))
        )
}
