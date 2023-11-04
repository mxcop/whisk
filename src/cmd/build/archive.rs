use std::{path::PathBuf, process::Command};

use owo_colors::colors::BrightRed;

use crate::{cmd::result::CmdResult, werror, term::{color::print_label, log_verbose}};

/// ### Archiver
/// Create an archive of the final object files. (static library)
pub fn archive(p: &PathBuf, v: bool, src: Vec<PathBuf>, pname: &String) -> CmdResult<()> {
    // Create the archive command.
    let mut cmd = Command::new("ar");
    cmd.arg("rcs"); // Flags

    // Create output directory.
    let out_dir = p.join("./bin/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("archiver", "failed to create output directory."));
    }
    cmd.arg(out_dir.join(format!("lib{}.a", pname)));

    // Add all the object files.
    let obj_dir = p.join("./bin/obj/");
    let mut id = 0u32;
    for file in src {
        let obj_file = obj_dir.join(format!("{}_{}.o", id, file.file_stem().unwrap_or_default().to_string_lossy()));
        cmd.arg(obj_file);
        id += 1;
    }

    // Verbose logging.
    if v {
        log_verbose(&pname, &cmd);
    }

    // Spawn the process.
    let Ok(output) = cmd.output() else {
        return Err(werror!("archiver", "failed to spawn archiver process."));
    };

    if !output.status.success() {
        print_label::<BrightRed>("ERROR", &obj_dir, &pname, None);
        // TODO: improve error msg.
        return Err(werror!("archiver", "error while archiving static lib."));
    }

    Ok(())
}
