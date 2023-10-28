use std::{path::PathBuf, process::Command};

use anstyle::AnsiColor;

use crate::{cmd::result::CmdResult, werror, term::color::print_label};

/// ### Linker
/// Link together the final object files into an executable.
pub fn link(p: &PathBuf, compiler: &String, src: Vec<PathBuf>, libs: &Option<Vec<String>>, lib: &Option<Vec<String>>, pname: &String) -> CmdResult<()> {
    // Create the link command.
    let mut cmd = Command::new(&compiler);

    // Create output directory.
    let out_dir = p.join("./bin/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("Failed to create output directory"));
    }
    cmd.arg("-o");
    cmd.arg(out_dir.join(pname));

    // Add all the object files.
    let obj_dir = p.join("./bin/obj/");
    let mut id = 0u32;
    for file in src {
        let obj_file = obj_dir.join(format!("{}_{}.o", id, file.file_stem().unwrap().to_string_lossy()));
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

    // Spawn the process.
    let Ok(mut process) = cmd.spawn() else {
        return Err(werror!("Failed to spawn linker process"));
    };

    // Wait for process to finish.
    let Ok(status) = process.wait() else {
        return Err(werror!("Failed to get linker process exit status"));
    };

    if !status.success() {
        print_label(AnsiColor::BrightRed, "ERROR", &obj_dir, &pname, None);
        // TODO: improve error msg.
        return Err(werror!("Error while building"));
    }

    Ok(())
}

pub fn link_slib(p: &PathBuf, src: Vec<PathBuf>, pname: &String) -> CmdResult<()> {
    // Create the archive command.
    let mut cmd = Command::new("ar");
    cmd.arg("rcs"); // Flags

    // Create output directory.
    let out_dir = p.join("./bin/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("Failed to create output directory"));
    }
    cmd.arg(out_dir.join(format!("lib{}.a", pname)));

    // Add all the object files.
    let obj_dir = p.join("./bin/obj/");
    let mut id = 0u32;
    for file in src {
        let obj_file = obj_dir.join(format!("{}_{}.o", id, file.file_stem().unwrap().to_string_lossy()));
        cmd.arg(obj_file);
        id += 1;
    }

    // Spawn the process.
    let Ok(mut process) = cmd.spawn() else {
        return Err(werror!("Failed to spawn linker process"));
    };

    // Wait for process to finish.
    let Ok(status) = process.wait() else {
        return Err(werror!("Failed to get linker process exit status"));
    };

    if !status.success() {
        print_label(AnsiColor::BrightRed, "ERROR", &obj_dir, &pname, None);
        // TODO: improve error msg.
        return Err(werror!("Error while building"));
    }

    Ok(())
}
