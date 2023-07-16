rust
fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
    y
}
