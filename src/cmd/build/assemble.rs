use std::{path::PathBuf, process::Command};

use owo_colors::colors::{BrightGreen, BrightRed};

use crate::{cmd::result::CmdResult, werror, term::color::print_label};

/// ### Assembler (-c)
/// Assemble all changed files into object files.
pub fn assemble(p: &PathBuf, compiler: &String, pre_files: Vec<PathBuf>) -> CmdResult<()> {
    let mut args: Vec<String> = Vec::with_capacity(32);

    // Output ".o" object files.
    args.push("-c".to_owned());
    // args.push("-w".to_owned());
    // args.push("-std=c++17".to_owned());
    // args.push("-msse4.2".to_owned());

    // Create output directory.
    let out_dir = p.join("./bin/obj/");
    if std::fs::create_dir_all(&out_dir).is_err() {
        return Err(werror!("assembler", "failed to create output directory."));
    }

    let mut threads = Vec::new();
    for file in pre_files {
        let compiler = compiler.clone();
        let args = args.clone();
        let out_dir = out_dir.clone();
        let pwd = p.clone();

        // Create a new thread for assembling.
        let handle = std::thread::spawn(
            move || assembler_thread(pwd, out_dir, file, compiler, args)
        );

        threads.push(handle);
    }

    for handle in threads {
        handle.join().expect("[Preprocessing] Fatal error, failed to join thread!")?;
    }

    Ok(())
}

/// ### Assembler thread
/// Thread for processing a single preprocessor file.
fn assembler_thread(pwd: PathBuf, out_dir: PathBuf, file: PathBuf, compiler: String, args: Vec<String>) -> CmdResult<()> {
    // Create compile command.
    let mut cmd = Command::new(&compiler);
    cmd.args(args.iter());

    let Some(file_name) = file.file_stem() else {
        return Err(werror!("assembler", "missing file name."));
    };

    cmd.arg("-o");
    let out_file = out_dir.join(format!("{}.o", file_name.to_string_lossy()));
    cmd.arg(&out_file);

    // Spawn process.
    cmd.arg(&file);
    let timer = std::time::SystemTime::now();
    let Ok(output) = cmd.output() else {
        return Err(werror!("assembler", "failed to spawn compiler process."));
    };
    let time = timer.elapsed().unwrap_or_default().as_millis() as u32;

    // Get some logging info.
    let Some(parent) = file.parent() else {
        return Err(werror!("assembler", "failed to get parent of path `{}`.", file.to_string_lossy()));
    };
    let file_path = parent.strip_prefix(&pwd).unwrap_or(parent).to_path_buf();
    let full_file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();

    // Return with error if the compiler returned unsuccessful.
    if !output.status.success() {
        print_label::<BrightRed>("ERROR", &file_path, &full_file_name, None);
        return Err(werror!("assembler", "error while compiling `{}`.\n{}", file.to_string_lossy(), String::from_utf8_lossy(&output.stderr)));
    }

    print_label::<BrightGreen>("DONE", &file_path, &full_file_name, Some(time));

    Ok(())
}
