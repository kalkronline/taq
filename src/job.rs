use std::future::Future;
use std::pin::Pin;

// this module wraps these core message passing types
type PinnedFut<'a, R> = Pin<Box<dyn Future<Output = R> + Send + 'a>>;
type FutGen<A, R> = Box<dyn for<'a> FnOnce(&'a mut A) -> PinnedFut<'a, R> + Send>;

/// Create a [`Job`] using a closure-style syntax.
///
/// # Note
/// The "closure" is actually an async block, so using
/// `await` is allowed.
///
/// # Examples
/// ```
/// let request: Job<String, ()> = job!(|s| {
///    *s = "Hello, world!".to_string();
/// });
///
/// let request: Job<String, String> = job!(|s| s.clone());
/// ```
#[macro_export]
macro_rules! job {
    ( |$a:ident$(: $t:ty)?| $b:stmt ) => {
        $crate::Job::new(Box::new(|$a$(: $t)?| Box::pin(async move { $b })))
    };
    ( |$a:ident$(: $t:ty)?| $b:tt ) => {
        $crate::Job::new(Box::new(|$a$(: $t)?| Box::pin(async move $b)))
    };
    ( move |$a:ident$(: $t:ty)?| $b:stmt ) => {
        $crate::Job::new(Box::new(move |$a$(: $t)?| Box::pin(async move { $b })))
    };
    ( move |$a:ident$(: $t:ty)?| $b:tt ) => {
        $crate::Job::new(Box::new(move |$a$(: $t)?| Box::pin(async move $b)))
    };
}

/// Do work on a mutable reference through a closure.
///
/// # Example
/// ```
/// async fn string_provider(req: Job<String, ()>) {
///     let mut s = String::new();
///     req.with(&mut s).await;
///     assert_eq!(s, "Hello, world!");
/// }
///
/// string_provider(job!(|s| {
///     *s = "Hello, world!".to_string();
/// })).await;
/// ```
pub struct Job<A, R>(FutGen<A, R>);

impl<A, R> Job<A, R> {
    /// Creates a new [`Job`] from a specific type of closure.
    ///
    /// # Note
    /// Unless you have a reason not to, use the [`job!`] macro.
    pub fn new(f: FutGen<A, R>) -> Self {
        Self(f)
    }

    /// Consumes the [`Job`] and runs the inner closure using
    /// the provided reference.
    pub async fn with(self, a: &mut A) -> R {
        (self.0)(a).await
    }
}
