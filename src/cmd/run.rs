use std::{process::Command, path::PathBuf};

use anstyle::{Style, AnsiColor};
use clap::ArgMatches;
use crate::{werror, cfg::ProConfig};

use super::{result::CmdResult, build};

/// Build & run a mix C/C++ project.
pub fn run(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let path = args.get_one::<PathBuf>("path").expect("Missing server name in `new` command.");
    
    // Read project config file.
    let toml_path = path.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(werror!("`whisk.toml` not found in `{}`", path.to_str().unwrap_or("-")));
    };

    // Parse project config file.
    let cfg: ProConfig = toml::from_str(&toml).unwrap();

    // Build the project.
    build::build(args)?;

    // Print:
    let blu_style = Style::new().fg_color(Some(AnsiColor::BrightBlue.into()));
    let dimmed = Style::new().dimmed();

    println!("\n{}[{} {}Running{} {}]{}", 
        dimmed.render(), dimmed.render_reset(), 
        blu_style.render(), blu_style.render_reset(), 
        dimmed.render(), dimmed.render_reset());

    // Spawn the compiled binary.
    let process = Command::new(path.join(format!("./bin/{}", cfg.package.name)))
        .spawn().expect("failed to run");
    let _ = process.wait_with_output();

    Ok(())
}
