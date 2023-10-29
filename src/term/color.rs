use std::path::PathBuf;

use anstyle::{Style, AnsiColor};

/// Whisk print dimmed macro. (use same as [std::format])
#[macro_export]
macro_rules! printd {
    ($fmt_str:literal) => {{
        let style = anstyle::Style::new().dimmed();
        print!("{}{}{}", style.render(), $fmt_str, style.render_reset());
    }};

    ($fmt_str:literal, $($args:expr),*) => {{
        let style = anstyle::Style::new().dimmed();
        print!("{}{}{}", style.render(), format!($fmt_str, $($args),*), style.render_reset());
    }};
}

/// Print a status line.
pub fn print_status(color: AnsiColor, label: &str, status: &str, ctx: Option<&str>) {
    let label_style = Style::new().fg_color(Some(color.into())).bold();

    print!("~ {}{}{}",
        label_style.render(),
        label,
        label_style.render_reset()
    );

    print!(" {}", status);

    if let Some(ctx) = ctx {
        printd!(" ({})", ctx);
    }
    println!();
}

/// Print a label line.
pub fn print_label(color: AnsiColor, label: &str, path: &PathBuf, file_name: &String, time: Option<u32>) {
    let label_style = Style::new().fg_color(Some(color.into())).bold();

    print!("  {}{}{}",
        label_style.render(),
        label,
        label_style.render_reset()
    );

    printd!(" {}\\", path.to_string_lossy().replace("/", "\\"));

    print!("{file_name}");

    if let Some(time) = time {
        printd!(" ({}ms)", time);
    }
    println!();
}
