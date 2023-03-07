use crate::{Handle, Task};
use state::Container;

static GLOBAL: Container![Send + Sync] = <Container![Send + Sync]>::new();

/// Gets a global [`Handle`] to a task of type `T`.
///
/// Returns `None` if the global state doesn't have a handle
pub fn get_handle<T>() -> Option<&'static Handle<T>>
where
    T: Task + Send + Sync + 'static,
{
    GLOBAL.try_get()
}

pub(crate) fn set<T>(item: Handle<T>)
where
    T: Task + Send + Sync + 'static,
{
    let res = GLOBAL.set(item);
    if !res {
        panic!("can't do that");
    }
}
