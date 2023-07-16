rust
fn test(_: impl for<'a> MyFn<&'a A, Output=IntoIter<&'a A>>) {
    let x = A;
    f.call(&x);
}
