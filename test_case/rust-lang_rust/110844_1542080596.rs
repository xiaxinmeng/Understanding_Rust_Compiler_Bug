rust
fn test<'a>() -> impl Sized + 'a {
    let r: &'static i32 = test();
    r
}
