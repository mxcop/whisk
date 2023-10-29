use regex::{Captures, CaptureMatches};

#[derive(Debug)]
pub struct CompilerOutput {
    pub output: Vec<CompilerMsg>
}

#[derive(Debug)]
pub enum CompilerMsg {
    Compiler(CompilerInfo),
    Linker(LinkerInfo)
}

#[derive(Debug)]
pub struct CompilerInfo {
    pub filepath: String,
    pub filename: String,
	pub line: i32,
	pub column: i32,
	pub itype: String,
	pub text: String,
	pub code: String,
	pub ptr: String,

	pub parent_fn: String
}

#[derive(Debug)]
pub struct LinkerFirstDef {
    pub filename: String,
	pub line: i32
}

#[derive(Debug)]
pub struct LinkerInfo {
    pub filename: String,
	pub line: i32,
	pub column: i32,
	pub itype: String,
	pub subtype: String,
	pub affected_symbol: String,
	pub text: String,

	pub start_idx: i32,
	pub end_idx: i32,
	pub parent_fn: String,

    pub first_def: LinkerFirstDef
}

/// Capture get macro for strings. (c.get(n))
macro_rules! strget {
    ($caps:expr, $idx:expr) => {{
        $caps.get($idx).map(|cap| cap.as_str().to_string()).unwrap_or_default()
    }};
}

/// Capture get macro for integers. (c.get(n))
macro_rules! i32get {
    ($caps:expr, $idx:expr) => {{
        $caps.get($idx).map(|cap| cap.as_str().to_string().parse::<i32>().unwrap()).unwrap_or_default()
    }};
}

impl<'r, 'h> From<CaptureMatches<'r, 'h>> for CompilerOutput {
    fn from(cmatches: CaptureMatches<'r, 'h>) -> Self {
        let mut output: Vec<CompilerMsg> = Vec::with_capacity(32);

        for caps in cmatches {
            output.push(CompilerMsg::Compiler(CompilerInfo { 
                filepath: strget!(caps, 3).replace("/", "\\"),
                filename: strget!(caps, 1),
                line: i32get!(caps, 4),
                column: i32get!(caps, 5),
                itype: strget!(caps, 6),
                text: strget!(caps, 7),
                code: strget!(caps, 8),
                ptr: strget!(caps, 9),
                parent_fn: strget!(caps, 2),
            }));
        }

        Self { output }
    }
}
