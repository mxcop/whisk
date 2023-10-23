mod args;
mod cfg;

mod file;
mod cmd;

fn main() {
    // Parse arguments.
    let cmd = args::cli();
    let matches = cmd.get_matches();

    // Execute subcommand.
    let res = match matches.subcommand() {
        Some(("new", matches)) => todo!(),
        Some(("build", matches)) => cmd::build::build(matches),
        Some(("run", matches)) => cmd::run::run(matches),
        _ => unreachable!(),
    };

    // Print any errors that occurred.
    if let Err(err) = res {
        println!("{err}");
    }
}
