
error[E0NNN]: the trait `Pattern<'_>` is not implemented for `&char`
 --> src/main.rs:3:45
  |
3 |     s.chars().filter(|c| !(" \n\t".contains(c))).collect::<String>();
  |                                             ^
  |
  = note: an `impl<'a, F> Pattern<'a> for F` exists but is not satisified...
  = note: because `&char` does not implement `FnOnce<(char,)>`...
  = note: an `impl<'_, A, F> FnOnce<A> for &'_ F ` exists but is not satisified...
  = note: because `char` does not implement `FnOnce<(char,)>`
