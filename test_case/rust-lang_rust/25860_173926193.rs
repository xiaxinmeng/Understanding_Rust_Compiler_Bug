 rust
fn bad<'a, 'b>(g: fn (&'a &'b i32) -> i32) {
    let _f: fn(x: &'b &'b i32) -> i32 = g;
}
