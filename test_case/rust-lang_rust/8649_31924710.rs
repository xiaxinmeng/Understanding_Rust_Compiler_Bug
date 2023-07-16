 rust
{ // in for loop expansion:
    let iter = ::std::iter::assert_iter(&mut rhs);
    ...
}
// in std::iter
pub fn assert_iter<'a, T, I: Iterator<T>>(iter: &'a mut I) -> &'a mut I {iter}
