rust
#[derive(Copy, Clone)]
struct Flags;

trait A {
}

impl<T> Drop for T where T: A {
    fn drop(&mut self) {
    }
}
