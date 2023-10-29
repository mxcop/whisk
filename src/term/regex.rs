pub mod exports {
    pub use once_cell;
    pub use regex;
}

#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::term::regex::exports::once_cell::sync::OnceCell<$crate::term::regex::exports::regex::Regex> =
            $crate::term::regex::exports::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| $crate::term::regex::exports::regex::Regex::new($re).unwrap())
    }};
}
