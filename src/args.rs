use std::path::PathBuf;

use clap::{arg, Command};

/// Whisk CLI clap command.
pub fn cli() -> Command {
    Command::new("whisk")
        .about("Whisk, a simplistic build system for C/C++")
        .author("Max <mxcop>, mxcop.dev@gmail.com")
        .arg(arg!(-v --verbose "Turn on verbose logging (debug)").global(true))
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new whisk project")
                .arg(arg!(name: <NAME> "New project name").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("tree")
                .about("Make a dependency tree of a whisk project")
                .arg(
                    arg!(path: <PATH> "Project path")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf))
                        .default_value("."),
                )
        )
        .subcommand(
            Command::new("build")
                .about("Build a whisk project")
                .arg(
                    arg!(path: <PATH> "Project path")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf))
                        .default_value("."),
                )
                .arg(
                    arg!(target: <TARGET> "Build target")
                        .required(false)
                        .value_parser(clap::value_parser!(String)),
                )
        )
        .subcommand(
            Command::new("run")
                .about("Build and run a whisk project")
                .arg(
                    arg!(path: <PATH> "Project path")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf))
                        .default_value("."),
                )
        )
        .subcommand(
            Command::new("clean")
                .about("Remove all build files of a whisk project")
                .arg(
                    arg!(path: <PATH> "Project path")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf))
                        .default_value("."),
                )
        )
}
