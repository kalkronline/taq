//! Task management abstraction.

/// Error types and what not.
pub mod error;
/// Global task handles.
pub mod global;
mod handle;
mod job;
mod manager;
mod task;

pub use handle::{Handle, HandleExt};
pub use job::Job; // and job macro
pub use manager::TaskManager;
pub use task::{run, run_global, Task};
