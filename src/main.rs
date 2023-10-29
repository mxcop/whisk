mod args;
mod cfg;

mod gcc;
mod term;
mod file;
mod cmd;

fn main() {
    // Parse arguments.
    let cmd = args::cli();
    let matches = cmd.get_matches();

    // Execute subcommand.
    let res = match matches.subcommand() {
        Some(("new", _matches)) => todo!(),
        Some(("build", matches)) => cmd::build::build(matches),
        Some(("run", matches)) => cmd::run::run(matches),
        Some(("clean", matches)) => cmd::clean::clean(matches),
        _ => unreachable!(),
    };

    // Print any errors that occurred.
    if let Err(err) = res {
        println!("{err}");
    }
}
