rust
fn f1_with_named_generic<F: FnMut(u32) -> u32, T: Iterator<Item=u32>>(a: u32) -> impl FnOnce(F) -> T {
    move |f: F| {
        (0..a).map(f)
    }
}
