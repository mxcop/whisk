use std::{path::PathBuf, process::Command};

use owo_colors::colors::BrightRed;

use crate::{cmd::result::CmdResult, werror, term::{color::print_label, log_verbose}};

/// ### Linker
/// Link together the final object files into an executable.
pub fn link(p: &PathBuf, v: bool, compiler: &String, src: Vec<PathBuf>, libs: &Option<Vec<String>>, lib: &Option<Vec<String>>, pname: &String) -> CmdResult<()> {
    // Create the link command.
    let mut cmd = Command::new(&compiler);

    // Create output directory.
    let out_dir = p.join("./bin/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("linker", "failed to create output directory."));
    }

    cmd.arg("-o");
    cmd.arg(out_dir.join(pname));

    // Add all the object files.
    let obj_dir = p.join("./bin/obj/");
    let mut id = 0u32;
    for file in src {
        let obj_file = obj_dir.join(format!("{}_{}.o", id, file.file_stem().unwrap_or_default().to_string_lossy()));
        cmd.arg(obj_file);
        id += 1;
    }

    // Add library directories.
    if let Some(libs) = libs {
        for lib in libs {
            cmd.arg(format!("-L{}", p.join(lib).to_string_lossy()));
        }
    }

    // Add libraries.
    if let Some(libs) = lib {
        for lib in libs {
            cmd.arg(format!("-l{}", lib));
        }
    }

    // Verbose logging.
    if v {
        log_verbose(&pname, &cmd);
    }

    // Spawn the process.
    let Ok(output) = cmd.output() else {
        return Err(werror!("linker", "failed to spawn linker process."));
    };

    if !output.status.success() {
        print_label::<BrightRed>("ERROR", &obj_dir, &pname, None);
        // TODO: improve error msg.
        return Err(werror!("linker", "error while linking!\n{}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}
