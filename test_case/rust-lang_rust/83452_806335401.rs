rust
use std::ops::Add;

const fn foo<T: Add>() {
    // The function definition doesn't matter WRT its bounds, so I'm not writing it.
}

const fn bar<T: ?const Add> (l: T, r: T) {
    foo::<T>()
}
