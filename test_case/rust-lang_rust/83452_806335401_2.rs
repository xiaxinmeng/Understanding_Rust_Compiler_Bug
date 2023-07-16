rust
fn foo<T: std::ops::Add>() {
    // The function definition doesn't matter
}

fn bar<T> (l: T, r: T) {
    foo::<T>()
}
