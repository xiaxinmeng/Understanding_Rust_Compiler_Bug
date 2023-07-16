rust
fn foo(make_iter: impl for<'a> Fn(&'a mut [i32]) -> Box<dyn Iterator<Item = &'a mut i32> + 'a>) {
    let mut data = [1, 2, 3, 4];

    make_iter(&mut data);
}
