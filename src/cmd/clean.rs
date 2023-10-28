use std::path::PathBuf;

use clap::ArgMatches;

use crate::werror;

use super::result::CmdResult;

/// Clean a whisk C/C++ project.
pub fn clean(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing server name in `new` command.");

    let timer = std::time::SystemTime::now();

    if std::fs::remove_dir_all(pwd.join("./bin/")).is_err() {
        return Err(werror!("Failed to delete bin directory"));
    }

    let time = timer.elapsed().unwrap().as_secs_f32();
    println!("Cleaning finished in {:.2}s", time);
    
    Ok(())
}
