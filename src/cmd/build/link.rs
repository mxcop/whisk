use std::{path::PathBuf, process::Command};

use crate::{cmd::result::CmdResult, werror};

/// ### Linker
/// Link together the final object files into an executable.
pub fn link(p: &PathBuf, compiler: &String, src: Vec<PathBuf>, pname: &String) -> CmdResult<()> {
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

    // Spawn the process.
    let Ok(mut process) = cmd.spawn() else {
        return Err(werror!("Failed to spawn linker process"));
    };

    // Wait for process to finish.
    let Ok(status) = process.wait() else {
        return Err(werror!("Failed to get linker process exit status"));
    };

    if !status.success() {
        // TODO: improve error msg.
        return Err(werror!("Error while building"));
    }

    Ok(())
}
