use std::path::PathBuf;

use anstyle::{Style, AnsiColor};
use clap::ArgMatches;
use spinoff::Spinner;
use crate::{cfg::ProConfig, werror};

use super::result::CmdResult;

mod compile;

/// Build a mix C/C++ project.
pub(crate) fn build(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let path = args.get_one::<PathBuf>("path").expect("Missing server name in `new` command.");

    // Read project config file.
    let toml_path = path.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(werror!("`whisk.toml` not found in `{}`", path.to_str().unwrap_or("-")));
    };

    // Parse project config file.
    let project_cfg: ProConfig = toml::from_str(&toml).unwrap();
    let mag_style = Style::new().fg_color(Some(AnsiColor::BrightMagenta.into()));
    let grn_style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into()));
    let dimmed = Style::new().dimmed();

    { // Pretty print.
        let abs_str = std::fs::canonicalize(&path).unwrap().to_string_lossy().to_string();
        let abs_path = abs_str.trim_start_matches("\\\\?\\"); // Remove Windows prefix

        println!("{}Compiling{}   ~ {}{}{} {}({}){}", 
            mag_style.render(), mag_style.render_reset(), 
            grn_style.render(), &project_cfg.package.name, grn_style.render_reset(), 
            dimmed.render(), abs_path, dimmed.render_reset());
    }
    
    let mut spinner = Spinner::new(spinoff::spinners::Dots, "Compiling...", spinoff::Color::Magenta);

    // Compilation timer.
    let timer = std::time::SystemTime::now();

    // Execute the compiler.
    compile::compile(path, project_cfg)?;

    // Record compile time.
    let time = timer.elapsed().unwrap().as_millis();

    spinner.success(&format!("Compilation finished in {}ms!", time));

    Ok(())
}
