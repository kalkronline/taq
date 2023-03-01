use crate::error::{TaqError, TaqResult};
use crate::{futgen, FutGen};
use tokio::sync::{mpsc, oneshot};

#[async_trait::async_trait]
pub trait Task {
    async fn task(self, mgr: TaskManager<Self>) -> Option<()>
    where
        Self: Sized;
}

/// Utility provided to a `Task` implementer to assist in outer communication.
///
/// In order for the `Handler` to work properly, the `Task` implementer must
/// call `self_req` in a loop, immediately call and await the `FutGen`, and
/// stop the task when `self_req` returns `None`.
///  
/// # Example
/// ```
/// use mantask::{Task, TaskManager};
/// use tokio::time::sleep;
/// use std::time::Duration;
///
/// struct NothingTask;
///
/// // Task does nothing on its own.
/// #[async_trait::async_trait]
/// impl Task for NothingTask {
///     async fn task(mut self, mut mgr: TaskManager<Self>) {
///         while let Some(give) = mgr.self_req().await {
///             give(&mut self).await;
///         }
///     }
/// }
///
/// struct GreetTask;
///
/// // Friendlier task.
/// #[async_trait::async_trait]
/// impl Task for GreetTask {
///     async fn task(mut self, mut mgr: TaskManager<Self>) {
///         loop {
///             tokio::select! {
///                 _ = sleep(Duration::from_secs(1)) => {
///                    println!("Hello, world!");
///                 },
///                 give = mgr.self_req() => give(&mut self).await,
///             }
///         }
///     }
/// }
/// ```
pub struct TaskManager<T>(mpsc::UnboundedReceiver<FutGen<T, ()>>);

impl<T: Task> TaskManager<T> {
    pub async fn self_req(&mut self) -> Option<FutGen<T, ()>> {
        self.0.recv().await
    }
}

#[derive(Clone)]
pub struct Handle<T>(mpsc::UnboundedSender<FutGen<T, ()>>);

impl<T: Send + 'static> Handle<T> {
    pub fn send(&self, func: FutGen<T, ()>) -> TaqResult<()> {
        self.0.send(func).map_err(|_| TaqError::Send)?;
        Ok(())
    }

    pub fn run<R: Send + 'static>(&self, func: FutGen<T, R>) -> TaqResult<oneshot::Receiver<R>> {
        let (tx, rx) = oneshot::channel();

        self.send(futgen!(|t| {
            let _ = tx.send(func.give(t).await);
        }))?;

        Ok(rx)
    }
}

/// Spawns a `Task`, and returns its `Handle`.
///
/// # Example
/// ```
/// ```
pub fn spawn<T>(task: T) -> Handle<T>
where
    T: Task + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded_channel();
    let (handle, manager) = (Handle(tx), TaskManager(rx));

    tokio::spawn(async move {
        task.task(manager).await;
    });

    handle
}
