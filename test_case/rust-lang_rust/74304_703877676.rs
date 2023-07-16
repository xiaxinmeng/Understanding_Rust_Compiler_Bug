rust
pub trait Wake: Clone {
    fn wake(self);

    fn wake_by_ref(&self) { ... }
}
