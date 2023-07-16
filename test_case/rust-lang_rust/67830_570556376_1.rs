rust
fn test(f: impl for<'a> MyFn<&'a A, Output=impl Iterator + 'a>) {
    let x = A;
    f.call(&x);
}
