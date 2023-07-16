 rust
fn plus<T>(a: &T, b: &T) -> T where
    for <'a, 'b> &'a T: Add<&'b T, Output=T>,
{
    a + b
}
