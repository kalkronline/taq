use crate::error::{TaqError, TaqResult};
use crate::job::Job;
use crate::task::Task;
use tokio::sync::mpsc;

mod ext;
pub use ext::HandleExt;

type Tx<T> = mpsc::UnboundedSender<Job<T, ()>>;

/// Utility for sending [`Jobs`](crate::Job) to a [`Task`](crate::Task).
///
/// This struct implements [`Clone`], allowing multiple
/// handles to be created for the same task.
pub struct Handle<A: Task>(Tx<A>);

impl<A> Handle<A>
where
    A: Task + Send + 'static,
{
    pub(crate) fn new(tx: Tx<A>) -> Self {
        Self(tx)
    }

    /// Sends a [`Job`] to the task's
    /// [`TaskManager`](crate::TaskManager).
    ///
    /// # Note
    /// Sending a job to a task does not garuntee that the job
    /// will be executed. The task implementation may not poll the
    /// [`TaskManager`](crate::TaskManager) or execute its jobs.
    /// Additionally the Task may be dropped immediately after
    /// receiving the job.
    ///
    /// # Errors
    /// If the [`TaskManager`](crate::TaskManager) has been
    /// dropped, this method will return an error.
    ///
    /// # Example
    /// ```
    /// // handle: Handle<String>
    /// handle.run(job!(|s: &mut String| s.clone()))?;
    /// ```
    pub fn run(&self, func: Job<A, ()>) -> TaqResult<()> {
        self.0.send(func).map_err(|_| TaqError::SendToClosed)?;
        Ok(())
    }
}

impl<T: Task> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
