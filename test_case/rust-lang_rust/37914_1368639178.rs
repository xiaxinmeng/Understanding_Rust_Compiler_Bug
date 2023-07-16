
error: the `min` method cannot be invoked on a trait object
    --> f57.rs:2:9
     |
2    |      *t.min().unwrap()
     |         ^^^
     |
    ::: /Users/ekuber/workspace/rust/library/core/src/iter/traits/iterator.rs:3012:15
     |
3012 |         Self: Sized,
     |               ----- this has a `Sized` requirement
     |
     = note: you need `&mut dyn Iterator<Item = &u64>` instead of `&dyn Iterator<Item = &u64>`
