rust
trait Take<T> {
    fn take(&self) -> Option<T>;
}

impl<T: Copy> Take<T> for Cell<Option<T>> {
    fn take(&self) -> Option<T> {
        let value = self.get();
        self.set(None);
        value
    }
}
