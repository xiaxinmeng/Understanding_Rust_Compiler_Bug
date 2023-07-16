rust
fn f<'a, 'b>(x: i32) -> (&'a i32, &'b i32) {
    let y = &x;
    (y, y)
}
