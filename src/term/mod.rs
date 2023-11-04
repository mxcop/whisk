use std::process::Command;

use owo_colors::OwoColorize;

pub mod color;

pub fn log_verbose(file_name: &str, cmd: &Command) {
    let mut log = String::new();
    log.push_str(&format!("   {} {file_name} ~ '{}", "LOG".dimmed(), &cmd.get_program().to_string_lossy()));
    
    for arg in cmd.get_args() {
        log.push_str(&format!(" {}", &arg.to_string_lossy()));
    }
    println!("{log}'");
}
