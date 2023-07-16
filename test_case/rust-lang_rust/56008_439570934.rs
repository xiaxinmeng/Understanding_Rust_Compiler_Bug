rust
use std::marker::PhantomData;

#[derive(Copy, Clone)]
struct SpookyThing<T> {
    phantom: PhantomData<*const T>,
}

#[derive(Clone)]
struct NotCopyable;

fn main() {
    let scary = SpookyThing::<NotCopyable> {
        phantom: PhantomData,
    };
    creep(scary, scary);
}

fn creep<T>(_creep1: SpookyThing<T>, _creep2: SpookyThing<T>) {}
