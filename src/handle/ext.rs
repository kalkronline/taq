use crate::{error::TaqResult, job, Handle, Job, Task};
use std::sync::mpsc;
use tokio::sync::oneshot;

/// Extentions for [`Handle`].
pub trait HandleExt<A, R> {
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
    fn blocking_recv(&self, func: Job<A, R>) -> TaqResult<mpsc::Receiver<R>>;
}

#[doc(hidden)]
impl<A: Task + Send + 'static, R: Send + 'static> HandleExt<A, R> for Handle<A> {
    fn recv(&self, func: Job<A, R>) -> TaqResult<oneshot::Receiver<R>> {
        let (tx, rx) = oneshot::channel();

        self.run(job!(|t| let _ = tx.send(func.with(t).await)))?;

        Ok(rx)
    }

    fn blocking_recv(&self, func: Job<A, R>) -> TaqResult<mpsc::Receiver<R>> {
        let (tx, rx) = mpsc::sync_channel(1);

        self.run(job!(|t| let _ = tx.send(func.with(t).await)))?;

        Ok(rx)
    }
}
