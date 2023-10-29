use std::fmt;

use anstyle::{Style, AnsiColor};

pub type CmdResult<T> = std::result::Result<T, CmdError>;

/// Whisk sub-command error type.
#[derive(Debug, Clone)]
pub struct CmdError {
    /// Error message.
    msg: String,
    /// Error context.
    ctx: String
}

impl CmdError {
    pub fn from_msg(ctx: &str, msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            ctx: ctx.to_owned()
        }
    }
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_style = Style::new().fg_color(Some(AnsiColor::BrightRed.into())).bold();
        write!(f, "{}error{}: ", err_style.render(), err_style.render_reset())?;

        let dim_style = Style::new().dimmed();
        let ctx_style = Style::new().fg_color(Some(AnsiColor::BrightYellow.into())).italic();
        write!(f, "{}<{}{}{}{}{}>{} ", dim_style.render(), dim_style.render_reset(), ctx_style.render(), self.ctx, ctx_style.render_reset(), dim_style.render(), dim_style.render_reset())?;

        write!(f, "{}", self.msg)
    }
}

/// Whisk error macro.
#[macro_export]
macro_rules! werror {
    ($ctx_str:literal, $str:expr) => {{
        use crate::cmd::result::CmdError;
        CmdError::from_msg($ctx_str, $str)
    }};

    ($ctx_str:literal, $fmt_str:literal) => {{
        use crate::cmd::result::CmdError;
        CmdError::from_msg($ctx_str, $fmt_str)
    }};

    ($ctx_str:literal, $fmt_str:literal, $($args:expr),*) => {{
        use crate::cmd::result::CmdError;
        CmdError::from_msg($ctx_str, &format!($fmt_str, $($args),*))
    }};
}

/// Convert a toml parse result into a local cmd result.
pub fn toml_result<T>(result: std::result::Result<T, toml::de::Error>) -> CmdResult<T> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(werror!("manifest", &e.to_string())),
    }
}
