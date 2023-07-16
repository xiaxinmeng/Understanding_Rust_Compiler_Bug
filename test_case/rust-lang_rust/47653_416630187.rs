rust
struct Ptr<T: ?Sized>(*const T);

impl<T> Ptr<T> {
    fn cast<U>(self) -> Ptr<U> {
        Ptr(self.0 as *const _)
    }
}

impl<T> Ptr<[T]> {
    fn cast<U>(self) -> Ptr<[U]> {
        Ptr(self.0 as *const _)
    }
}
