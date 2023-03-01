use std::fmt;

pub type TaqResult<R> = Result<R, TaqError>;

use TaqError::*;

pub enum TaqError {
    /// An error occurred while sending a message to a task.
    Send,
}

impl fmt::Debug for TaqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Send => write!(f, "An error occurred while sending a message to a task."),
        }
    }
}
