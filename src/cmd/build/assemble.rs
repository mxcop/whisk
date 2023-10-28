use std::{path::PathBuf, process::Command};

use crate::{cmd::result::CmdResult, werror};

/// ### Assembler (-c)
/// Assemble all changed files into object files.
pub fn assemble(p: &PathBuf, compiler: &String, pre_files: Vec<PathBuf>) -> CmdResult<()> {
    let mut args: Vec<String> = Vec::with_capacity(32);

    // Output ".o" object files.
    args.push("-c".to_owned());

    // Create output directory.
    let out_dir = p.join("./bin/obj/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("Failed to create output directory"));
    }

    let mut threads = Vec::new();
    for file in pre_files {
        let compiler = compiler.clone();
        let args = args.clone();
        let out_dir = out_dir.clone();

        // Create a new thread for compiling.
        let handle = std::thread::spawn(move || {
            // Create compile command.
            let mut cmd = Command::new(&compiler);
            cmd.args(args.iter());

            let Some(file_name) = file.file_stem() else {
                return Err(werror!("Missing file name"));
            };

            cmd.arg("-o");
            let out_file = out_dir.join(format!("{}.o", file_name.to_string_lossy()));
            cmd.arg(&out_file);

            // Spawn process.
            cmd.arg(&file);
            let Ok(mut process) = cmd.spawn() else {
                return Err(werror!("Failed to spawn compiler process"));
            };
            let timer = std::time::SystemTime::now();

            // Wait for process to finish.
            let Ok(status) = process.wait() else {
                return Err(werror!("Failed to get compiler process exit status"));
            };
            let time = timer.elapsed().unwrap().as_millis();
            println!("Assembled {} in {}ms", &file_name.to_string_lossy().to_string(), time);

            if !status.success() {
                return Err(werror!("Error while compiling `{}`", file.to_string_lossy()));
            }

            Ok(())
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap()?;
    }

    Ok(())
}
