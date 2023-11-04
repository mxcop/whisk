use std::{process::Command, path::PathBuf, fs::canonicalize};

use clap::ArgMatches;
use owo_colors::colors::{BrightYellow, BrightGreen};
use crate::{werror, cfg::{ProConfig, PackageType}, term::color::print_status};

use super::{result::{CmdResult, toml_result}, build};

/// Build & run a whisk project.
pub fn run(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing path in `run` command.");
    if pwd.exists() == false {
        return Err(werror!("filesystem", "{:?} isn't a directory", pwd));
    }
    let bin_dir = pwd.join("./bin/");

    // Read project config file.
    let toml_path = pwd.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(werror!("`whisk.toml` not found in `{}`", pwd.to_str().unwrap_or("-")));
    };

    // Parse project config file.
    let cfg: ProConfig = toml_result(toml::from_str(&toml))?;

    // Exit if this is a library.
    if cfg.package.ptype == PackageType::Library {
        print_status::<BrightYellow>("Exiting ", &cfg.package.name, Some("cannot run library"));
        return Ok(());
    }

    // Build the project.
    build::build(args)?;

    // Gather binary output paths.
    let bin = bin_dir.join(&cfg.package.name);
    let mut abs_bin = canonicalize(&bin_dir).expect("Failed to get absolute bin directory")
        .to_string_lossy().trim_start_matches("\\\\?\\").to_owned().replace("/", "\\");

    abs_bin.push('\\');
    abs_bin.push_str(&cfg.package.name);
    #[cfg(target_os = "windows")]
    abs_bin.push_str(".exe");

    print_status::<BrightGreen>("Running ", &cfg.package.name, Some(&abs_bin));

    // Spawn the compiled binary.
    let process = Command::new(bin)
        .spawn().expect("failed to run");
    let _ = process.wait_with_output();

    Ok(())
}
