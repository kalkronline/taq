use std::future::Future;
use std::pin::Pin;

type PinnedFuture<'a, R> = Pin<Box<dyn Future<Output = R> + Send + 'a>>;
type FutureGenerator<A, R> = Box<dyn for<'a> FnOnce(&'a mut A) -> PinnedFuture<'a, R> + Send>;

#[macro_export]
macro_rules! futgen {
    ( |$a:ident$(: $t:ty)?| $b:stmt ) => {
        $crate::FutGen::new(Box::new(|$a$(: $t)?| Box::pin(async move { $b })))
    };
    ( |$a:ident$(: $t:ty)?| $b:tt ) => {
        $crate::FutGen::new(Box::new(|$a$(: $t)?| Box::pin(async move $b)))
    };
}

/// A wrapper around a function that returns a future.
///
/// # Overview
/// [`Handle`](crate::Handle) accepts a `FutGen` to send to a task, which provides
/// a mutable reference to itself, and returns a future.
///
///
/// # Example
/// ```
/// let futgen = FutGen::new(Box::new(|t: &mut String| {
///     Box::pin(async move {
///         *t = "Hello, world!".to_owned();
///     })
/// }));
///
/// let ezmode = futgen!(|t: &mut String| {
///     *t = "Hello, world!".to_string();
/// });
pub struct FutGen<A, R>(FutureGenerator<A, R>);

impl<A, R> FutGen<A, R> {
    pub fn new(f: FutureGenerator<A, R>) -> Self {
        Self(f)
    }

    pub async fn give(self, a: &mut A) -> R {
        (self.0)(a).await
    }
}

fn todo() {
    let futgen = FutGen::new(Box::new(|t: &mut String| {
        Box::pin(async move {
            *t = "Hello, world!".to_owned();
        })
    }));
}
