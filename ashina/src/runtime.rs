use std::future::Future as StdFuture;

pub trait Runtime {
    fn spawn<T>(future: T)
    where
        T: StdFuture + Send + 'static,
        T::Output: Send + 'static;
}
