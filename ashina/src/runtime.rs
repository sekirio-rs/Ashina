use futures03::future::BoxFuture;
use std::future::Future as StdFuture;

pub trait Runtime {
    fn spawn<T>(future: T) -> BoxFuture<'static, T::Output>
    where
        T: StdFuture + Send + 'static,
        T::Output: Send + 'static;
}
