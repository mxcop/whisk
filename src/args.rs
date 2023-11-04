use std::path::PathBuf;

use clap::{Command, arg};

// use clap::{Args, Parser, Subcommand, ValueEnum};

/// Whisk CLI clap command.
pub fn cli() -> Command {
    Command::new("whisk")
        .about("A simplistic build system for C/C++")
        .arg(arg!(-v --verbose ... "Turn verbose information on").global(true))
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Creates a new project")
                .arg(arg!(name: <NAME> "New project name").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("tree")
                .about("Makes tree of a project")
                .arg(arg!(path: <PATH> "Project path").required(false)
                    .value_parser(clap::value_parser!(PathBuf)).default_value("."))
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
        /* Global Verbose output argument */
        // .arg(arg!(verbose: -v --verbose ... "Use verbose output")
        //     .action(ArgAction::Append)
        //     .global(true)
        // )
        
}

// #[derive(Debug, Parser)] // requires `derive` feature
// #[command(name = "git")]
// #[command(about = "A fictional versioning CLI", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Debug, Subcommand)]
// enum Commands {
//     /// Clones repos
//     #[command(arg_required_else_help = true)]
//     New {
//     },
//     /// Compare two commits
//     Tree {
//     },
//     /// pushes things
//     #[command(arg_required_else_help = true)]
//     Push {
//         /// The remote to target
//         remote: String,
//     },
//     /// adds things
//     #[command(arg_required_else_help = true)]
//     Add {
//         /// Stuff to add
//         #[arg(required = true)]
//         path: Vec<PathBuf>,
//     },
//     Stash(StashArgs),
//     #[command(external_subcommand)]
//     External(Vec<OsString>),
// }

