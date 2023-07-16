rust
fn foo(make_iter: impl for<'a> Fn(&'a mut [i32]) -> impl Iterator<Item = &'a mut i32>) {
    let mut data = [1, 2, 3, 4];

    make_iter(&mut data);
}
