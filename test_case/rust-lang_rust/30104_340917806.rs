rust
fn push(mut node: ::std::cell::RefMut<(i32, i32)>) {
    let (ref mut left, ref mut right) = *node;
}
