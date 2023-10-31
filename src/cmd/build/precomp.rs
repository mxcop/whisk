use std::{path::PathBuf, sync::{Arc, Mutex}, process::Command};

use owo_colors::colors::{BrightRed, BrightGreen};

use crate::{cmd::result::CmdResult, werror, term::color::print_label};

/// ### Pre-compile (.gch)
/// Compile pre-compiled headers.
/// 
/// Returns a list of the pre-compiled header files.
pub fn precomp(p: &PathBuf, compiler: &String, src: Vec<PathBuf>, inc: &Option<Vec<String>>) -> CmdResult<Vec<PathBuf>> {
    let mut args: Vec<String> = Vec::with_capacity(32);
    
    // Output ".gch" compiled header files.
    args.push("-x".to_owned());
    args.push("c++-header".to_owned());
    args.push("-c".to_owned());

    // Add include directories.
    if let Some(inc) = inc {
        args.extend(inc.iter().map(|inc| inc.clone()));
    }

    // Create output directory.
    let gch_dir = p.join("./bin/gch/");
    if std::fs::create_dir_all(&gch_dir).is_err() {
        return Err(werror!("precomp", "failed to create pre-compile output directory."));
    }

    // List of changed files.
    let files: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::with_capacity(32)));

    let mut threads = Vec::new();

    for file in src {
        let pwd = p.clone();
        let gch_dir = gch_dir.clone();
        let compiler = compiler.clone();
        let args = args.clone();
        let files = Arc::clone(&files);

        // Create a new thread for compiling.
        let handle = std::thread::spawn(
            move || precomp_thread(pwd, gch_dir, file, compiler, args, files)
        );

        threads.push(handle);
    }

    for handle in threads {
        handle.join().expect("[Pre-compile] fatal error, failed to join thread!")?;
    }

    let Ok(mutex) = Arc::try_unwrap(files) else {
        return Err(werror!("precomp", "failed to unwrap atomic reference counter."));
    };

    // Save the output file path.
    if let Ok(outputs) = mutex.into_inner() {
        Ok(outputs)
    } else {
        Err(werror!("precomp", "failed to get inner value of mutex."))
    }
}

fn precomp_thread(pwd: PathBuf, gch_dir: PathBuf, file: PathBuf, compiler: String, args: Vec<String>, files: Arc<Mutex<Vec<PathBuf>>>) -> CmdResult<()> {
    // Create compile command.
    let mut cmd = Command::new(&compiler);
    cmd.args(args.iter());

    let Some(file_name) = file.file_name().map(|name| name.to_string_lossy()) else {
        return Err(werror!("precomp", "not a source file `{}`.", file.to_string_lossy()));
    };

    cmd.arg("-o");
    let out_filename = format!("{}.gch", &file_name);
    let out_file = gch_dir.join(&out_filename);
    cmd.arg(&out_file);

    cmd.arg(&file);

    // Spawn process.
    let timer = std::time::SystemTime::now();
    dbg!(&cmd);
    let Ok(output) = cmd.output() else {
        return Err(werror!("precomp", "failed to spawn pre-compile process."));
    };
    let time = timer.elapsed().unwrap_or_default().as_millis() as u32;
    
    let file_path = file.parent().unwrap().strip_prefix(&pwd).unwrap_or(file.parent().unwrap()).to_path_buf();

    if !output.status.success() {
        print_label::<BrightRed>("ERROR", &file_path, &file_name.to_string(), None);
        return Err(werror!("precomp", "Failed to process `{}`.", file.to_string_lossy()));
    }

    if let Ok(mut guard) = files.lock() {
        print_label::<BrightGreen>("DONE", &file_path, &file_name.to_string(), Some(time));
        guard.push(out_file);
        Ok(())
    } else {
        Err(werror!("precomp", "failed to save changed file path."))
    }
}
