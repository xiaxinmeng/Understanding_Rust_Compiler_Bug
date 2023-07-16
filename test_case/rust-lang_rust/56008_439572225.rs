rust
struct SpookyThing<T> {
    phantom: PhantomData<*const T>,
}

impl<T> Clone for SpookyThing<T> {
    fn clone(&self) -> Self {
        SpookyThing {
            phantom: PhantomData
        }
    }
}

impl<T> Copy for SpookyThing<T> {}
