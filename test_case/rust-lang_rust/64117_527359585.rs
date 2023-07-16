rust
fn take_iter<T : Iterator>(iter: T) {
    iter.consume(); // consumed
    // do something else
}
