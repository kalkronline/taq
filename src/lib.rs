pub mod error;
mod futgen;
mod taq;

pub use futgen::*;
pub use taq::*;

pub mod prelude {
    pub use crate::{Handle, Task, TaskManager};
}
