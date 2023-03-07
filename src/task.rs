use crate::handle::Handle;
use crate::manager::TaskManager;

use tokio::sync::mpsc;

/// An isolated process which can be managed with a [`Handle`].
///
/// # Note
/// Uses [`async_trait`](https://docs.rs/async-trait/0.1.64/async_trait/index.html)
/// to provide async trait methods.
#[async_trait::async_trait]
pub trait Task {
    /// Called by the [`run`](crate::run) function.
    ///
    /// # How to Implement
    /// Implementing [`Task`] requires some special attention when implementing
    /// in order for [`Handle`] to work as expected.
    ///
    /// For [`Job`](crate::Job) passing and execution to work, you must
    /// constantly poll the [`TaskManager`] and consume the [`Job`](crate::Job)
    /// on a `Some` variant. For task stopping when all [`Handles`](crate::Handle)
    /// are dropped, you must return when polling [`TaskManager`] returns the `None`
    /// variant.
    ///
    /// # Example Implementation
    /// ```
    /// struct DoesJobsAndStuff;
    ///
    /// impl DoesJobsAndStuff {
    ///     async fn stuff(&mut self) {
    ///         // stuff
    ///     }
    /// }
    ///
    /// #[async_trait::async_trait]
    /// impl Task for DoesJobsAndStuff {
    ///     async fn task(self, mut mgr: TaskManager<Self>) -> Option<()> {
    ///         loop {
    ///             tokio::select! {
    ///                 job = mgr.poll()   => job?.with(&mut self).await,
    ///                 _   = self.stuff() => (),
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    async fn task(self, mgr: TaskManager<Self>) -> Option<()>
    where
        Self: Sized;
}

/// Runs a [`Task`] and returns a [`Handle`] to it.
pub fn run<T>(task: T) -> Handle<T>
where
    T: Task + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded_channel();
    let (handle, manager) = (Handle::new(tx), TaskManager::new(rx));

    tokio::spawn(async move {
        task.task(manager).await;
    });

    handle
}

/// Runs a [`Task`] and saves the handle to the global state.
///
/// # Panics
/// ... if the global state has already been set.
pub fn run_global<T>(task: T)
where
    T: Task + Send + Sync + 'static,
{
    crate::global::set(run(task));
}
