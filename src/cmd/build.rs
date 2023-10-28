use std::path::PathBuf;

use clap::ArgMatches;
use crate::{cfg::ProConfig, werror};

use super::result::CmdResult;

mod preprocess;
mod assemble;
mod link;
// mod compile;

/// Build a mix C/C++ project.
pub fn build(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing server name in `new` command.");

    // Read project config file.
    let toml_path = pwd.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(werror!("`whisk.toml` not found in `{}`", pwd.to_str().unwrap_or("-")));
    };

    // Parse project config file.
    let cfg: ProConfig = toml::from_str(&toml).unwrap();

    // TODO: change this default...
    let compiler = cfg.profile.compiler.clone().unwrap_or("g++".into());

    // Gather the project files.
    let src_files = cfg.profile.source_args(&pwd)?;
    let inc_files = cfg.profile.include_args(&pwd)?;

    let timer = std::time::SystemTime::now();

    //--------------------------//
    //  [stage] Pre-processing  //
    //--------------------------//
    let pre_files = preprocess::preprocess(pwd, &compiler, src_files.clone(), &inc_files)?;

    let out_file = pwd.join(format!("./bin/{}.exe", &cfg.package.name));

    // Exit if no files were modified.
    if out_file.exists() && pre_files.is_empty() {
        let time = timer.elapsed().unwrap().as_secs_f32();
        println!("No changes found.");
        println!("Finished {} target in {:.2}s", &cfg.package.name, time);
        return Ok(());
    }

    //--------------------------//
    //  [stage] Assembling      //
    //--------------------------//
    assemble::assemble(pwd, &compiler, pre_files)?;

    //--------------------------//
    //  [stage] Linking         //
    //--------------------------//
    link::link(pwd, &compiler, src_files, &cfg.package.name)?;

    let time = timer.elapsed().unwrap().as_secs_f32();
    println!("Finished {} target in {:.2}s", &cfg.package.name, time);
    
    Ok(())
}
