rust
struct AllocRef<'a, A> {
    allocator: [&'a A; (size_of::<A>() > 0) as usize],
}

impl<'a, A: Allocator> Deref for AllocRef<'a, A> {
    type Target = A;

    fn deref(&self) -> &A {
        // This should be fine?
        self.allocator.get(0).unwrap_or_else(|| unsafe { NonNull::dangling().as_ref() })
    }
}

impl<'a, A> Copy for AllocRef<'a, A> {}

impl<'a, A> Clone for AllocRef<'a, A> {
    ...
}
