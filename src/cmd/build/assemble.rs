use std::{path::PathBuf, process::Command};

use anstyle::AnsiColor;

use crate::{cmd::result::CmdResult, werror, term::color::print_label};

/// ### Assembler (-c)
/// Assemble all changed files into object files.
pub fn assemble(p: &PathBuf, compiler: &String, pre_files: Vec<PathBuf>) -> CmdResult<()> {
    let mut args: Vec<String> = Vec::with_capacity(32);

    // Output ".o" object files.
    args.push("-c".to_owned());

    // Create output directory.
    let out_dir = p.join("./bin/obj/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("[Assembling] Failed to create output directory."));
    }

    let mut threads = Vec::new();
    for file in pre_files {
        let compiler = compiler.clone();
        let args = args.clone();
        let out_dir = out_dir.clone();
        let pwd = p.clone();

        // Create a new thread for compiling.
        let handle = std::thread::spawn(move || {
            // Create compile command.
            let mut cmd = Command::new(&compiler);
            cmd.args(args.iter());

            let Some(file_name) = file.file_stem() else {
                return Err(werror!("[Assembling] Missing file name."));
            };

            cmd.arg("-o");
            let out_file = out_dir.join(format!("{}.o", file_name.to_string_lossy()));
            cmd.arg(&out_file);

            // Spawn process.
            cmd.arg(&file);
            let Ok(mut process) = cmd.spawn() else {
                return Err(werror!("[Assembling] Failed to spawn compiler process."));
            };
            let timer = std::time::SystemTime::now();

            // Wait for process to finish.
            let Ok(status) = process.wait() else {
                return Err(werror!("[Assembling] Failed to get compiler process exit status."));
            };
            let time = timer.elapsed().unwrap_or_default().as_millis() as u32;

            // Get some logging info.
            let Some(parent) = file.parent() else {
                return Err(werror!("[Assembling] Failed to get parent of path `{}`.", file.to_string_lossy()));
            };
            let file_path = parent.strip_prefix(&pwd).unwrap_or(parent).to_path_buf();
            let full_file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();

            // Return with error if the compiler returned unsuccessful.
            if !status.success() {
                print_label(AnsiColor::BrightRed, "ERROR", &file_path, &full_file_name, None);
                return Err(werror!("[Assembling] Error while compiling `{}`.", file.to_string_lossy()));
            }

            print_label(AnsiColor::BrightGreen, "DONE", &file_path, &full_file_name, Some(time));

            Ok(())
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().expect("[Preprocessing] Fatal error, failed to join thread!")?;
    }

    Ok(())
}
