rust
// perma-unstable
pub fn check_assert_eq<'a, A, B>(a: &'a A, b: &'a B) -> Result<(), (&'a A, &'a B)>
    where A: PartialEq<B>
{
    if a == b { Ok(()) } else { Err((a, b)) }
}
macro_rules! assert_eq {
    ($left:expr, $right:expr) => ({
        if let Err((left_val, right_val)) = check_assert_eq(&$left, &$right) {
            panic!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#, left_val, right_val);
        }
    });
}
