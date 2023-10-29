use std::path::PathBuf;

use anstyle::AnsiColor;
use clap::ArgMatches;

use crate::{werror, term::color::print_status};

use super::result::CmdResult;

/// Clean a whisk C/C++ project.
pub fn clean(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing path in `clean` command.");

    let timer = std::time::SystemTime::now();

    if std::fs::remove_dir_all(pwd.join("./bin/")).is_err() {
        return Err(werror!("clean", "Failed to delete bin directory"));
    }

    let time = timer.elapsed().unwrap().as_secs_f32();
    print_status(AnsiColor::BrightGreen, "Cleaned", &format!("in {:.2}s", time), None);
    
    Ok(())
}
