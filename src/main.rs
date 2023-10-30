mod args;
mod cfg;

mod files;
mod term;
mod cmd;

use cmd::{build, run, clean};

fn main() {
    // Parse arguments.
    let cmd = args::cli();
    let matches = cmd.get_matches();

    // Execute subcommand.
    let res = match matches.subcommand() {
        Some(("new", _matches)) => todo!(),
        Some(("build", matches)) => build::build(matches),
        Some(("run", matches)) => run::run(matches),
        Some(("clean", matches)) => clean::clean(matches),
        _ => unreachable!(),
    };

    // Print any errors that occurred.
    if let Err(err) = res {
        println!("{err}");
    }
}
