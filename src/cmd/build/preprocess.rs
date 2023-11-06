use std::{path::PathBuf, sync::{Arc, Mutex}, process::Command};

use owo_colors::colors::{BrightRed, BrightGreen, BrightCyan};

use crate::{cmd::result::CmdResult, werror, term::{color::print_label, log_verbose}};

/// ### Preprocessor (-E)
/// Pre-process all project source files.
/// 
/// Returns a list of the pre-proccesed files that changed.
/// So we can avoid recompiling unchanged files.
pub fn preprocess(p: &PathBuf, v: bool, g_args: &Vec<String>, lang: &str, compiler: &String, src: Vec<PathBuf>, inc: &Option<Vec<String>>) -> CmdResult<Vec<PathBuf>> {
    let mut args: Vec<String> = Vec::with_capacity(32);
    args.append(&mut g_args.clone());
    args.push(format!("-x{lang}"));
    
    // Output ".i" preprocessed files.
    args.push("-E".to_owned());

    // Add include directories.
    if let Some(inc) = inc {
        args.extend(inc.iter().map(|inc| inc.clone()));
    }

    // Create output directory.
    let pre_dir = p.join("./bin/pre/");
    if std::fs::create_dir_all(&pre_dir).is_err() {
        return Err(werror!("preprocess", "failed to create pre-processing output directory."));
    }
    let obj_dir = p.join("./bin/obj/");

    // List of changed files.
    let changed_files: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::with_capacity(32)));

    let mut uid = 0u32;
    let mut threads = Vec::new();

    for file in src {
        let pwd = p.clone();
        let pre_dir = pre_dir.clone();
        let obj_dir = obj_dir.clone();
        let compiler = compiler.clone();
        let args = args.clone();
        let changed_files = Arc::clone(&changed_files);

        // Create a new thread for compiling.
        let handle = std::thread::spawn(
            move || preprocess_thread(pwd, v, pre_dir, obj_dir, file, uid, compiler, args, changed_files)
        );

        threads.push(handle);
        uid += 1;
    }

    for handle in threads {
        handle.join().expect("[Preprocessing] fatal error, failed to join thread!")?;
    }

    let Ok(mutex) = Arc::try_unwrap(changed_files) else {
        return Err(werror!("preprocess", "failed to unwrap atomic reference counter."));
    };

    // Save the output file path.
    if let Ok(outputs) = mutex.into_inner() {
        Ok(outputs)
    } else {
        Err(werror!("preprocess", "failed to get inner value of mutex."))
    }
}

fn preprocess_thread(pwd: PathBuf, v: bool, pre_dir: PathBuf, obj_dir: PathBuf, file: PathBuf, uid: u32, compiler: String, args: Vec<String>, changed_files: Arc<Mutex<Vec<PathBuf>>>) -> CmdResult<()> {
    // Create compile command.
    let mut cmd = Command::new(&compiler);
    cmd.args(args.iter());

    let Some(file_name) = file.file_stem().map(|stem| stem.to_string_lossy()) else {
        return Err(werror!("preprocess", "not a source file `{}`.", file.to_string_lossy()));
    };

    cmd.arg("-o");
    let out_filename = format!("{}_{}.ii", uid, &file_name);
    let out_file = pre_dir.join(&out_filename);
    cmd.arg(&out_file);

    // Save the previous output file for comparing later.
    let previous_file = std::fs::read_to_string(&out_file).unwrap_or_default();

    // Spawn process.
    cmd.arg(&file);
    
    // Verbose logging.
    if v {
        log_verbose(&file_name.to_string(), &cmd);
    }

    let timer = std::time::SystemTime::now();
    let Ok(output) = cmd.output() else {
        return Err(werror!("preprocess", "failed to spawn preprocessor process."));
    };
    let time = timer.elapsed().unwrap_or_default().as_millis() as u32;
    
    let file_path = file.parent().unwrap().strip_prefix(&pwd).unwrap_or(file.parent().unwrap()).to_path_buf();
    let full_file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();

    if !output.status.success() {
        print_label::<BrightRed>("ERROR", &file_path, &full_file_name, None);
        return Err(werror!("preprocess", "failed to process `{}`.\n{}", file.to_string_lossy(), String::from_utf8_lossy(&output.stderr)));
    }

    // Check if the associated object file exists.
    if !obj_dir.join(format!("{}.o", &out_file.file_stem().unwrap_or_default().to_string_lossy())).exists() {
        // Save the changed file.
        if let Ok(mut guard) = changed_files.lock() {
            print_label::<BrightGreen>("DONE", &file_path, &full_file_name, Some(time));
            guard.push(out_file);
            return Ok(());
        } else {
            return Err(werror!("preprocess", "failed to save changed file path."));
        }
    }

    let new_file = std::fs::read_to_string(&out_file).unwrap_or_default();

    // If the new file is different.
    if new_file != previous_file {
        // Save the changed file.
        if let Ok(mut guard) = changed_files.lock() {
            print_label::<BrightGreen>("DONE", &file_path, &full_file_name, Some(time));
            guard.push(out_file);
        } else {
            return Err(werror!("preprocess", "failed to save changed file path."));
        }
    } else {
        print_label::<BrightCyan>("SKIP", &file_path, &full_file_name, None);
    }

    Ok(())
}
