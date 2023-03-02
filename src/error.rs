use std::fmt;

/// Helper type for a `Result` with a `TaqError`.
pub type TaqResult<R> = Result<R, TaqError>;

use TaqError::*;

#[derive(Debug)]
pub enum TaqError {
    /// The operation could not be completed because the task has been closed.
    SendToClosed,
}

impl fmt::Display for TaqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendToClosed => write!(
                f,
                "The operation could not be completed because the task has been closed."
            ),
        }
    }
}

impl std::error::Error for TaqError {}
