rust
use std::ops::Add;

fn add_generic<A>() -> impl IntoIterator<Item = A> {
    vec![]
}

pub fn add_one() -> impl IntoIterator<Item = <u32 as Add<u32>>::Output> {
    add_generic()
}
