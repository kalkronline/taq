use crate::{Job, Task};
use tokio::sync::mpsc;

type Rx<T> = mpsc::UnboundedReceiver<Job<T, ()>>;

/// Utility provided to a [`Task`] implementer for managing incoming
/// [`Jobs`](crate::Job).
///
/// See the documentation for [`Task`] for usage details.
pub struct TaskManager<T: Task>(Rx<T>);

impl<T: Task> TaskManager<T> {
    pub(crate) fn new(rx: Rx<T>) -> Self {
        Self(rx)
    }

    /// Returns `Some<Job>` if a job is available, or `None` if there
    /// are no more [`Handles`](crate::Handle) remaining.
    pub async fn poll(&mut self) -> Option<Job<T, ()>> {
        self.0.recv().await
    }
}
