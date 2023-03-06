use crate::{error::TaqResult, job, Handle, Job, Task};
use tokio::sync::oneshot;

/// Simplifies message passing to [`Tasks`](crate::Task).
pub trait Recv<A, R> {
    /// Sends a [`Job`] to the task and return a receiver for its result.
    ///
    /// # Errors
    /// If the [`TaskManager`](crate::TaskManager) has been
    /// dropped, this method will return an error.
    ///
    /// # Example
    /// ```
    /// // handle: Handle<String>
    /// let res = handle.run(job!(|s: &mut String| s.clone()))?.await?;
    /// println!("handles string: {res}");
    /// ```
    fn recv(&self, func: Job<A, R>) -> TaqResult<oneshot::Receiver<R>>;
}

#[doc(hidden)]
impl<A: Task + Send + 'static, R: Send + 'static> Recv<A, R> for Handle<A> {
    fn recv(&self, func: Job<A, R>) -> TaqResult<oneshot::Receiver<R>> {
        let (tx, rx) = oneshot::channel();

        self.run(job!(|t| let _ = tx.send(func.with(t).await)))?;

        Ok(rx)
    }
}
