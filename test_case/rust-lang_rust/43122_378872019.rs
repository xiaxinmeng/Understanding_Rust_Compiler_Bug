rust
use std::hash::Hash;

// Somehow add annotations so that `generator` implements
// `Generator<Yield = Box<Hash>, Return = ()>`.
// As of now, `Box<i32>` gets deduced for the Yield type.
let mut generator = || {
    yield Box::new(123i32);
    yield Box::new("hello");
};
