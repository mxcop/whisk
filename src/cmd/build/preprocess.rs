use std::{path::PathBuf, sync::{Arc, Mutex}, process::Command};

use crate::{cmd::result::CmdResult, werror};

/// ### Preprocessor (-E)
/// Pre-process all project source files.
/// 
/// Returns a list of the pre-proccesed files that changed.
/// So we can avoid recompiling unchanged files.
pub fn preprocess(p: &PathBuf, compiler: &String, src: Vec<PathBuf>, inc: &Option<Vec<String>>) -> CmdResult<Vec<PathBuf>> {
    let mut args: Vec<String> = Vec::with_capacity(32);

    // Output ".i" preprocessed files.
    args.push("-E".to_owned());

    // Add include directories.
    if let Some(inc) = inc {
        args.extend(inc.iter().map(|inc| inc.clone()));
    }

    // Create output directory.
    let pre_dir = p.join("./bin/pre/");
    if std::fs::create_dir_all(&pre_dir).is_err() {
        return Err(werror!("Failed to create pre-processing output directory"));
    }
    let obj_dir = p.join("./bin/obj/");

    let changed_files: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::with_capacity(32)));

    let mut id = 0u32;
    let mut threads = Vec::new();

    let timer = std::time::SystemTime::now();
    for file in src {
        let args = args.clone();
        let pre_dir = pre_dir.clone();
        let obj_dir = obj_dir.clone();
        let changed_files = changed_files.clone();
        let compiler = compiler.clone();

        // Create a new thread for compiling.
        let handle = std::thread::spawn(move || {
            let uid = id;
            // Create compile command.
            let mut cmd = Command::new(&compiler);
            cmd.args(args.iter());

            let Some(file_name) = file.file_stem().map(|stem| stem.to_string_lossy()) else {
                return Err(werror!("Not a source file `{}`", file.to_string_lossy()));
            };

            cmd.arg("-o");
            let out_filename = format!("{}_{}.ii", uid, &file_name);
            let out_file = pre_dir.join(&out_filename);
            cmd.arg(&out_file);

            // Save the previous output file for comparing later.
            let previous_file = std::fs::read_to_string(&out_file).unwrap_or_default();

            // Spawn process.
            cmd.arg(&file);
            let Ok(mut process) = cmd.spawn() else {
                return Err(werror!("Failed to spawn preprocessor process"));
            };
            let timer = std::time::SystemTime::now();

            // Wait for process to finish.
            let Ok(status) = process.wait() else {
                return Err(werror!("Failed to get preprocessor process exit status"));
            };
            let time = timer.elapsed().unwrap().as_millis();
            println!("Preprocessed {} in {}ms", &file_name, time);

            if !status.success() {
                return Err(werror!("Error while preprocessing `{}`", file.to_string_lossy()));
            }

            // Check if the associated object file exists.
            if !obj_dir.join(format!("{}.o", &out_file.file_stem().unwrap().to_string_lossy())).exists() {
                // Save the changed file.
                if let Ok(mut guard) = changed_files.lock() {
                    guard.push(out_file);
                    return Ok(());
                } else {
                    return Err(werror!("Failed to save changed file path"));
                }
            }

            let new_file = std::fs::read_to_string(&out_file).unwrap_or_default();

            // If the new file is different.
            if new_file != previous_file {
                // Save the changed file.
                if let Ok(mut guard) = changed_files.lock() {
                    guard.push(out_file);
                } else {
                    return Err(werror!("Failed to save changed file path"));
                }
            }

            Ok(())
        });

        threads.push(handle);
        id += 1;
    }

    for handle in threads {
        handle.join().unwrap()?;
    }

    let time = timer.elapsed().unwrap().as_millis();
    println!("Preprocessed project in {}ms", time);

    // Save the output file path.
    if let Ok(outputs) = Arc::try_unwrap(changed_files).unwrap().into_inner() {
        Ok(outputs)
    } else {
        Err(werror!("Failed to save output file path"))
    }
}
