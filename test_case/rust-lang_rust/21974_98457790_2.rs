 rust
fn plus<'c, T>(a: &'c T, b: &'c T) -> T where
    for <'a: 'c> &'a T: Add<&'a T, Output=T>,
{
    let s = a + b;
    &s + &s
}
