rust
pub trait SmartPointer {
    type P<T, A: Allocator = Global>;

    fn new<T>(t: T) -> Self::P<T>;

    fn new_in<T, A>(t: T, alloc: A) -> Self::P<T, A>;
}
