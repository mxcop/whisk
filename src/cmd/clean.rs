use std::path::PathBuf;

use clap::ArgMatches;
use owo_colors::colors::BrightGreen;

use crate::{werror, term::color::print_status};

use super::result::CmdResult;

/// Clean a whisk project.
pub fn clean(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing path in `clean` command.");
    if pwd.exists() == false {
        return Err(werror!("filesystem", "{:?} isn't a directory", pwd));
    }

    let timer = std::time::SystemTime::now();

    if std::fs::remove_dir_all(pwd.join("./bin/")).is_err() {
        return Err(werror!("clean", "Failed to delete bin directory"));
    }

    let time = timer.elapsed().unwrap().as_secs_f32();
    print_status::<BrightGreen>("Cleaned", &format!("in {:.2}s", time), None);
    
    Ok(())
}
