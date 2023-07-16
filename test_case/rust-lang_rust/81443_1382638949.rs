rust
use core::pin::Pin;
use std::future::Future;

struct Wrapper<T>(T);
impl<T> Wrapper<T>
where
    T: Future,
{
    fn run(mut self) {
        self.0.poll(todo!());
    }
}
