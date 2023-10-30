use std::path::PathBuf;

use owo_colors::{OwoColorize, Color};

/// Print a status line.
pub fn print_status<C>(label: &str, status: &str, ctx: Option<&str>)
where
    C: Color,
{
    print!("~ {} {}", label.fg::<C>().bold(), status);

    if let Some(ctx) = ctx {
        print!(" {}", format!("({})", ctx).dimmed());
    }
    println!();
}

/// Print a label line.
pub fn print_label<C>(label: &str, path: &PathBuf, file_name: &String, time: Option<u32>)
where
    C: Color,
{
    print!(
        "  {} {}\\{file_name}",
        label.fg::<C>().bold(),
        path.to_string_lossy().replace("/", "\\")
    );

    if let Some(time) = time {
        print!(" {}", format!("({}ms)", time).dimmed());
    }
    println!();
}
