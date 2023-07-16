rust
fn foo<'a, I: Iterator<Item = &'a mut i32>>(make_iter: impl Fn(&'a mut [i32]) -> I) {
    let mut data = [1, 2, 3, 4];

    make_iter(&mut data);
}
