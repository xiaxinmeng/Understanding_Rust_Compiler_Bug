 rust
fn explicit_types<'a, 'b>(callback: fn (&'a i32, &'b i32) -> i32 where 'a: 'b) {
    ...
}
