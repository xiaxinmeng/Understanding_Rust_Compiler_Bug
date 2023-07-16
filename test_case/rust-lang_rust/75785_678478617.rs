rust
pub fn async_fn(x: &mut i32) -> (&i32, &i32) {
    let y = &*x;
    *x += 1;
    (&32, y)
}
