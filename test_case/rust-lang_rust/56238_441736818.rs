rust
fn foo<'a: 'c, 'b: 'c, 'c>(
    x: &'a mut i32,
    y: &'b mut i32,
) -> impl Future<Output = i32> + Captures<'a> + Captures<'b> + 'c {
    async move {
        *x += *y;
        *y += *x;
        *x + *y
    }
}
