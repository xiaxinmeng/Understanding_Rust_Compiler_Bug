
error[E0277]: the trait bound `u8: std::ops::FnMut<(char,)>` is not satisfied
 --> <anon>:2:28
  |
2 |  let v: Vec<_> = "1, 2, 3".split(b',').collect();
  |                            ^^^^^ the trait `std::ops::FnMut<(char,)>` is not implemented for `u8`
  |
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `u8`

error[E0277]: the trait bound `u8: std::ops::FnOnce<(char,)>` is not satisfied
 --> <anon>:2:28
  |
2 |  let v: Vec<_> = "1, 2, 3".split(b',').collect();
  |                            ^^^^^ the trait `std::ops::FnOnce<(char,)>` is not implemented for `u8`
  |
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `u8`

error: no method named `collect` found for type `std::str::Split<'_, u8>` in the current scope
 --> <anon>:2:40
  |
2 |  let v: Vec<_> = "1, 2, 3".split(b',').collect();
  |                                        ^^^^^^^
  |
  = note: the method `collect` exists but the following trait bounds were not satisfied: `u8 : std::str::pattern::Pattern`, `std::str::Split<'_, u8> : std::iter::Iterator`
