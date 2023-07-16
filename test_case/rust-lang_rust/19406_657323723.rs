
error[E0277]: expected a `std::ops::FnMut<(char,)>` closure, found `u8`
 --> src/main.rs:2:38
  |
2 |      let v: Vec<_> = "1, 2, 3".split(b',').collect();
  |                                      ^^^^ expected an `FnMut<(char,)>` closure, found `u8`
  |
  = help: the trait `std::ops::FnMut<(char,)>` is not implemented for `u8`
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `u8`

error[E0599]: no method named `collect` found for struct `std::str::Split<'_, u8>` in the current scope
   --> src/main.rs:2:44
    |
2   |      let v: Vec<_> = "1, 2, 3".split(b',').collect();
    |                                            ^^^^^^^ method not found in `std::str::Split<'_, u8>`
    |
    = note: the method `collect` exists but the following trait bounds were not satisfied:
            `u8: std::str::pattern::Pattern<'_>`
            which is required by `std::str::Split<'_, u8>: std::iter::Iterator`
            `std::str::Split<'_, u8>: std::iter::Iterator`
            which is required by `&mut std::str::Split<'_, u8>: std::iter::Iterator`
