use std::{path::PathBuf, fs::canonicalize};

use clap::ArgMatches;
use owo_colors::colors::{BrightBlue, BrightCyan, BrightYellow, BrightMagenta, BrightGreen};
use crate::{man::{WhiskManifest, PackageType}, werror, term::color::print_status, cmd::result::{CmdResult, toml_result}};

mod preprocess;
mod assemble;
mod link;
mod archive;

/// Build a whisk project.
pub fn build(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing path in `build` command.");
    if pwd.exists() == false {
        return Err(werror!("filesystem", "{:?} isn't a directory", pwd));
    }
    let abs = canonicalize(&pwd).expect("Failed to get absolute project path").to_string_lossy().to_string();
    let abs = abs.trim_start_matches("\\\\?\\").to_owned().replace("/", "\\");

    let v = match args
        .get_one::<u8>("verbose")
        .expect("Issue with clap [verbose flag]")
    {
        0 => false,
        _ => true
    };

    // Read project config file.
    let toml_path = pwd.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(werror!("filesystem", "`whisk.toml` not found in {:?}", pwd));
    };

    // Parse project config file.
    let cfg: WhiskManifest = toml_result(toml::from_str(&toml))?;

    // TODO: change this default...
    let target = &cfg.package.target;
    let compiler = target.compiler.clone().unwrap_or("g++".into());
    let language = cfg.package.get_lang();

    // Gather the project files.
    let src_files = cfg.source_args(&pwd)?;
    let inc_files = cfg.include_args(&pwd)?;

    let timer = std::time::SystemTime::now();

    //--------------------------//
    //  [stage] Pre-processing  //
    //--------------------------//
    print_status::<BrightBlue>("Preprocess", &cfg.package.name, Some(&abs));
    let pre_files = preprocess::preprocess(pwd, v, language, &compiler, src_files.clone(), &inc_files)?;

    let out_file = match cfg.package.ptype {
        PackageType::Executable => pwd.join(format!("./bin/{}.exe", &cfg.package.name)),
        PackageType::Library => pwd.join(format!("./bin/lib{}.a", &cfg.package.name)),
    };

    // Exit if no files were modified.
    if out_file.exists() && pre_files.is_empty() {
        let time = timer.elapsed().unwrap().as_secs_f32();
        println!();
        print_status::<BrightCyan>("No changes", &cfg.package.name, Some(&format!("{:.2}s", time)));
        return Ok(());
    }

    //--------------------------//
    //  [stage] Assembling      //
    //--------------------------//
    println!();
    print_status::<BrightYellow>("Assembling", &cfg.package.name, Some(&abs));
    assemble::assemble(pwd, v, language, &compiler, pre_files)?;

    //--------------------------//
    //  [stage] Linking         //
    //--------------------------//
    println!();
    print_status::<BrightMagenta>("Linking ", &cfg.package.name, Some(&abs));
    match cfg.package.ptype {
        PackageType::Executable => link::link(pwd, v, &compiler, src_files, &target.libs, &target.lib, &cfg.package.name)?,
        PackageType::Library => archive::archive(pwd, v, src_files, &cfg.package.name)?
    };
    
    let time = timer.elapsed().unwrap().as_secs_f32();
    print_status::<BrightGreen>("Finished", &cfg.package.name, Some(&format!("{:.2}s", time)));
    
    Ok(())
}
