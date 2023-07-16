 rust
struct Foo<T>;
mod bar {
    impl super::Foo<()> { // impl allowed outside of Foo's module.
        fn bar(&self) -> Option<()> { None }
    }
}

impl Foo<bool> {
    // same method name, allowed only because Self cannot overlap
    // with any other impl with the same method (as with trait impls).
    fn bar(&self) -> Option<bool> { Some(true) }
}

// anonymous impls would be checked by coherence as if they were
// implementing traits from outside the current crate.
impl<T> [Foo<T>] {
    fn extra(&self) -> usize { 0 }
}
impl<T> Vec<Foo<T>> {
    fn extra(&self) -> usize { self.capacity() - self.len() }
}
