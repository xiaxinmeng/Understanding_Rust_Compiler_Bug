
struct Box<'a, T> {
    arena_lifetime: PhantomData<&'a ()>,
    ptr: Unique<T>,
}
