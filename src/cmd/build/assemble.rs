use std::{path::PathBuf, process::Command};

use anstyle::AnsiColor;

use crate::gcc::msg::CompilerOutput;
use crate::regex;
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
        return Err(werror!("Failed to create output directory"));
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
                return Err(werror!("Missing file name"));
            };

            cmd.arg("-o");
            let out_file = out_dir.join(format!("{}.o", file_name.to_string_lossy()));
            cmd.arg(&out_file);

            // Spawn process.
            cmd.arg(&file);
            let timer = std::time::SystemTime::now();
            let Ok(output) = cmd.output() else {
                return Err(werror!("Failed to spawn compiler process"));
            };
            
            // Wait for process to finish.
            // let Ok(output) = process.wait_with_output() else {
            //     return Err(werror!("Failed to get compiler process exit status"));
            // };
            let time = timer.elapsed().unwrap().as_millis() as u32;

            let file_path = file.parent().unwrap().strip_prefix(&pwd).unwrap_or(file.parent().unwrap()).to_path_buf();
            let full_file_name = file.file_name().unwrap().to_string_lossy().to_string();

            if !output.status.success() {
                print_label(AnsiColor::BrightRed, "ERROR", &file_path, &full_file_name, None);

                // let deep_regex = regex!(r"/([^:^\n]+):(\d+):(\d+):\s(\w+\s*\w*):\s(.+)\n(\s+)\d*\s*[|]*\s*(.*)\s+[|]*\s*\^+/gm");
                let simple_regex = regex!(r"([\w.]*?):.*?'(.*?)':.*?[\r\n](.*?):(\d*?):(\d*?): (.*?): (.*)[\r\n](.*)[\r\n](.*)");

                let mut stderr = String::from_utf8(output.stderr.clone()).unwrap();
                stderr.push_str(&stderr.clone());
                
                let out: CompilerOutput = simple_regex.captures_iter(&stderr).into();
                dbg!(out);

                return Err(werror!("Error while compiling `{}`", file.to_string_lossy()));
            }

            print_label(AnsiColor::BrightGreen, "DONE", &file_path, &full_file_name, Some(time));

            Ok(())
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap()?;
    }

    Ok(())
}
