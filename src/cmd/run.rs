use std::process::Command;

use anstyle::{Style, AnsiColor};
use clap::ArgMatches;
use super::{result::CmdResult, build};

/// Build & run a mix C/C++ project.
pub(crate) fn run(args: &ArgMatches) -> CmdResult<()> {
    // Build the project.
    build::build(args)?;

    let blu_style = Style::new().fg_color(Some(AnsiColor::BrightBlue.into()));
    let dimmed = Style::new().dimmed();

    println!("\n{}[{} {}Running{} {}]{}", 
        dimmed.render(), dimmed.render_reset(), 
        blu_style.render(), blu_style.render_reset(), 
        dimmed.render(), dimmed.render_reset());

    let process = Command::new("./bin/mix-test")
        .spawn().expect("failed to run");
    let _ = process.wait_with_output();

    Ok(())
}
