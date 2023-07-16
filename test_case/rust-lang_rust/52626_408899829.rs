rust
trait HashingContext {
    fn with_allocation<T>(&self, f: impl FnOnce(&Allocation) -> T) -> T;
}
