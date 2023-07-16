 rust
/// A "write once" cell.
pub struct Ivar<T:Copy> {
    data: Cell<Option<T>>
}

impl<T:Copy> Ivar<T> {
    pub fn get(&self) -> Option<T> {
        self.data.get()
    }

    pub fn fulfill(&self, value: T) {
        assert!(self.data.get().is_none());
        self.data.set(value);
    }

    pub fn unwrap(&self) -> T {
        self.get().unwrap()
    }
}
