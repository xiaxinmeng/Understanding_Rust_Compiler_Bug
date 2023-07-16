
error[E0277]: expected a `FnMut<(char,)>` closure, found `u8`
 --> src/main.rs:2:38
  |
2 |      let v: Vec<_> = "1, 2, 3".split(b',').collect();
  |                                ----- ^^^^ expected an `FnMut<(char,)>` closure, found `u8`
  |                                |
  |                                required by a bound introduced by this call
  |
  = help: the trait `FnMut<(char,)>` is not implemented for `u8`
  = help: the following other types implement trait `Pattern<'a>`:
            &'b String
            &'b [char; N]
            &'b [char]
            &'b str
            &'c &'b str
            [char; N]
            char
            pattern::MultiCharEqPattern<C>
  = note: required because of the requirements on the impl of `Pattern<'_>` for `u8`
note: required by a bound in `core::str::<impl str>::split`

error[E0599]: the method `collect` exists for struct `std::str::Split<'_, u8>`, but its trait bounds were not satisfied
 --> src/main.rs:2:44
  |
2 |        let v: Vec<_> = "1, 2, 3".split(b',').collect();
  |                                              ^^^^^^^ method cannot be called on `std::str::Split<'_, u8>` due to unsatisfied trait bounds
  |
  = note: the following trait bounds were not satisfied:
          `u8: Pattern<'_>`
          which is required by `std::str::Split<'_, u8>: Iterator`
          `std::str::Split<'_, u8>: Iterator`
          which is required by `&mut std::str::Split<'_, u8>: Iterator`
