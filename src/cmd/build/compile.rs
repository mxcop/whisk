use std::{process::Command, path::PathBuf, sync::{Mutex, Arc}};

use crate::{cmd::result::CmdResult, cfg::ProConfig, file::walk::{get_files, get_dirs}, werror};

/// Assemble all source files into object files.
fn assemble(p: &PathBuf, cfg: ProConfig) -> CmdResult<Vec<PathBuf>> {
    // Get the compiler from the project config.
    let compiler = cfg.profile.compiler.unwrap_or("g++".to_owned()); // TODO: change this default...
    let mut args: Vec<String> = Vec::with_capacity(32);

    // Output ".o" object files.
    args.push("-c".to_owned());

    // Set the include directories.
    if let Some(include_dirs) = cfg.profile.include {
        let includes = get_dirs(&include_dirs)?;
        for include in includes {
            args.push("-I".to_owned());
            args.push(p.join(include).to_string_lossy().to_string());
        }
    }

    // Create output directory.
    let out_dir = p.join("./bin/obj/");
    let _ = std::fs::remove_dir_all(&out_dir);
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("Failed to create output directory"));
    }

    // Get all the source files to compile.
    let files = get_files(p, &cfg.profile.src)?;
    let output: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::with_capacity(32)));

    let mut id = 0u32;
    let mut threads = Vec::new();
    for file in files {
        let compiler = compiler.clone();
        let args = args.clone();
        let out_dir = out_dir.clone();
        let output = output.clone();

        // Create a new thread for compiling.
        let handle = std::thread::spawn(move || {
            let uid = id;
            // Create compile command.
            let mut cmd = Command::new(&compiler);
            cmd.args(args.iter());

            let Some(file_name) = file.file_stem() else {
                return Err(werror!("Missing file name"));
            };

            cmd.arg("-o");
            let out_file = out_dir.join(format!("{}_{}.o", uid, file_name.to_string_lossy()));
            cmd.arg(&out_file);

            // Spawn process.
            cmd.arg(&file);
            let Ok(mut process) = cmd.spawn() else {
                return Err(werror!("Failed to spawn compiler process"));
            };

            // Wait for process to finish.
            let Ok(status) = process.wait() else {
                return Err(werror!("Failed to get compiler process exit status"));
            };

            if !status.success() {
                return Err(werror!("Error while compiling `{}`", file.to_string_lossy()));
            }

            // Save the output file path.
            if let Ok(mut guard) = output.lock() {
                guard.push(out_file);
            } else {
                return Err(werror!("Failed to save output file path"));
            }
            println!("Build `{}` succesfully!", file.to_string_lossy());

            Ok(())
        });
        threads.push(handle);
        id += 1;
    }

    for handle in threads {
        handle.join().unwrap()?;
    }

    // Save the output file path.
    if let Ok(outputs) = Arc::try_unwrap(output).unwrap().into_inner() {
        Ok(outputs)
    } else {
        Err(werror!("Failed to save output file path"))
    }
}

/// Assemble the source files seperately and then link them together.
pub(crate) fn compile(p: &PathBuf, cfg: ProConfig) -> CmdResult<()> {
    let cfg_clone = cfg.clone();

    // Create the link command.
    let compiler = cfg.profile.compiler.unwrap_or("g++".to_owned());
    let mut cmd = Command::new(&compiler);

    // Create output directory.
    let out_dir = p.join("./bin/");
    let _ = std::fs::remove_dir_all(&out_dir);
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("Failed to create output directory"));
    }
    cmd.arg("-o");
    cmd.arg(out_dir.join(cfg.package.name));

    let obj_files = assemble(p, cfg_clone)?;

    // Add all the object files.
    cmd.args(obj_files);

    // Spawn the process.
    let Ok(mut process) = cmd.spawn() else {
        return Err(werror!("Failed to spawn compiler process"));
    };

    // Wait for process to finish.
    let Ok(status) = process.wait() else {
        return Err(werror!("Failed to get compiler process exit status"));
    };

    if !status.success() {
        // TODO: improve error msg.
        return Err(werror!("Error while building"));
    }

    Ok(())
}
