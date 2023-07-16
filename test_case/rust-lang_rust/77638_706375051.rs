rust
use std::ops::Add;

trait A {
    type U: Add<i64> + Add<u64>;
}

fn f<T: A>(t: T::U) {
    let x = t + Default::default();
}
