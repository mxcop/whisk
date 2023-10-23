use std::fmt;

pub type CmdResult<T> = std::result::Result<T, CmdError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct CmdError {
    msg: String
}

impl CmdError {
    pub fn from_msg(msg: &str) -> Self {
        Self {
            msg: msg.to_owned()
        }
    }
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Issue: {}", self.msg)
    }
}
