rust
fn g<'a, T: 'a>(t: &T) -> &'a i32 {
    &0
}

fn f<'a, 'b, 'c: 'a + 'b, T: 'c>(x: T) -> (&'a i32, &'b i32) {
    // compare with returning (&'a i32, &'a i32)
    let y = g(&x);
    (y, y)
}
