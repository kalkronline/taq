//! Task management abstraction.

/// Error types and what not.
pub mod error;
mod handle;
mod job;
mod manager;
mod task;

pub use handle::ext;
pub use handle::Handle;
pub use job::Job; // and job macro
pub use manager::TaskManager;
pub use task::{run, Task};
