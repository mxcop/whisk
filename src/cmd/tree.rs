use std::{path::PathBuf, process::Command};

use clap::ArgMatches;

use crate::{werror, man::WhiskManifest, cmd::{result::toml_result, target}};

use super::result::CmdResult;

#[derive(Debug)]
/// Dependency tree
pub struct File {
    pub obj: String,
    pub src: String,
    pub deps: Vec<String>
}

impl File {
    pub fn new() -> Self {
        Self {
            obj: String::new(),
            src: String::new(),
            deps: Vec::with_capacity(16)
        }
    }
}

pub fn gen_tree(args: &ArgMatches) -> CmdResult<()> {
    // Retrieve CLI arguments.
    let pwd = args.get_one::<PathBuf>("path").expect("Missing path in `build` command.");

    let v = *args
        .get_one::<bool>("verbose")
        .expect("Issue with clap [verbose flag]");

    // Read project config file.
    let toml_path = pwd.join("whisk.toml");
    let Ok(toml) = std::fs::read_to_string(toml_path) else {
        return Err(werror!("filesystem", "`whisk.toml` not found in `{}`", pwd.to_str().unwrap_or("-")));
    };

    // Parse project config file.
    let cfg: WhiskManifest = toml_result(toml::from_str(&toml))?;

    // Get build target information for this package.
    let target = target::get_target_info(&cfg, args.get_one::<String>("target"), v);
    
    // TODO: change this default...
    let compiler = target.compiler.clone().unwrap_or("g++".into());

    // Gather the project files.
    let src_files = target.source_args(&pwd)?;
    let inc_files = target.include_args(&pwd)?;

    // Create compile command.
    let mut cmd = Command::new(&compiler);

    cmd.arg("-MM");
    cmd.args(inc_files.unwrap_or_default());
    cmd.args(src_files);

    let timer = std::time::SystemTime::now();
    let Ok(output) = cmd.output() else {
        return Err(werror!("tree", "failed to spawn compiler process."));
    };
    let _time = timer.elapsed().unwrap_or_default().as_millis() as u32;

    if output.status.success() == false {
        return Err(werror!("tree", "{}", String::from_utf8_lossy(&output.stderr)));
    }

    let result = String::from_utf8_lossy(&output.stdout);

    /* Parse compiler output */
    let mut files: Vec<File> = Vec::new();
    let mut file = File::new();
    let mut dep = String::new();
    let mut idx: u32 = 0;
    let mut chars = result.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            ' ' => {
                idx += 1;
                if idx == 2 {
                    file.src = dep.replace("\\", "/");
                }
                else if idx > 2 {
                    file.deps.push(dep.replace("\\", "/"));
                }
                dep = String::new();
            },
            '\n' => {
                idx = 0;
                file.deps.push(dep.replace("\\", "/"));
                dep = String::new();
                files.push(file);
                file = File::new();
            },
            '\\' if Some(&'\n') == chars.peek() || Some(&'\r') == chars.peek() => {
                while let Some(a) = chars.peek() { if a.is_ascii_whitespace() { chars.next(); } else { break; } } // skip whitespace
            },
            ':' => {/* ignore ':' char */},
            _ => {
                if idx == 0 {
                    file.obj.push(ch);
                } else {
                    dep.push(ch);
                }
            }
        }
    }

    for file in &files {
        println!("<{}> : {}", file.obj, file.src);
        for dep in &file.deps {
            println!("  {}", dep);
        }
    }

    println!("\n{:#?}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
