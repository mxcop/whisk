use std::{path::{PathBuf, Path}, process::Command};

use anstyle::{Style, AnsiColor};
use clap::ArgMatches;
use spinoff::Spinner;
use crate::cfg::ProConfig;

use super::result::{CmdResult, CmdError};

/// Build a mix C/C++ project.
pub(crate) fn build(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let path = args.get_one::<PathBuf>("path").expect("Missing server name in `new` command.");

    // Read project config file.
    let toml_path = path.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(CmdError::from_msg(
            &format!("`whisk.toml` not found in `{}`", path.to_str().unwrap_or("-"))
        ));
    };

    // Parse project config file.
    let project_cfg: ProConfig = toml::from_str(&toml).unwrap();
    let mag_style = Style::new().fg_color(Some(AnsiColor::BrightMagenta.into()));
    let grn_style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into()));
    let dimmed = Style::new().dimmed();

    { // Pretty print.
        let abs_str = std::fs::canonicalize(&path).unwrap().to_string_lossy().to_string();
        let abs_path = abs_str.trim_start_matches("\\\\?\\"); // Remove Windows prefix

        println!("{}Parsing{}   ~ {}{}{} {}({}){}", 
            mag_style.render(), mag_style.render_reset(), 
            grn_style.render(), &project_cfg.project.name, grn_style.render_reset(), 
            dimmed.render(), abs_path, dimmed.render_reset());
    }
    
    let mut spinner = Spinner::new(spinoff::spinners::Dots, "Parsing...", spinoff::Color::Magenta);

    // TODO: improve error handling here!
    std::env::set_current_dir(path).unwrap();
    std::fs::create_dir_all("./bin/").unwrap();

    // Create the compile command.
    let compiler = project_cfg.profile.cxx.unwrap_or("g++".to_owned());
    let mut cmd = Command::new(&compiler);

    // Add compiler arguments.
    cmd.args(project_cfg.profile.src);

    cmd.arg("-o");
    cmd.arg(Path::new("./bin/").join(&project_cfg.project.name));

    if let Some(include_dirs) = project_cfg.profile.inc {
        cmd.arg("-I");
        cmd.args(include_dirs);
    }

    spinner.clear();
    { // Pretty print.
        println!("{}Compiling{} ~ {}{}{} {}({}){}", 
            mag_style.render(), mag_style.render_reset(), 
            grn_style.render(), &project_cfg.project.name, grn_style.render_reset(), 
            dimmed.render(), compiler, dimmed.render_reset());
    }
    spinner = Spinner::new(spinoff::spinners::Dots, "Compiling...", spinoff::Color::Magenta);

    // Compilation timer.
    let timer = std::time::SystemTime::now();

    // Execute the compiler.
    let process = cmd.spawn().expect("failed to spawn compiler");
    let _ = process.wait_with_output();

    // Record compile time.
    let time = timer.elapsed().unwrap().as_millis();

    // std::thread::sleep(std::time::Duration::from_millis(3000));

    spinner.success(&format!("Compilation finished in {}ms!", time));

    // { // Pretty print.
    //     let abs_str = std::fs::canonicalize(Path::new("./bin/")).unwrap().to_string_lossy().to_string();
    //     let abs_path = abs_str.trim_start_matches("\\\\?\\"); // Remove Windows prefix

    //     println!("\n{}Output dir{} : ({})", 
    //         grn_style.render(), grn_style.render_reset(), 
    //         abs_path);
    // }

    Ok(())
}
