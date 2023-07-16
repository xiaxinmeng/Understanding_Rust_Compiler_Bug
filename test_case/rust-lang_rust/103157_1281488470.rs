
error[E0277]: the trait bound `f64: Eq` is not satisfied
 --> <source>:4:11
  |
1 | #[derive(PartialEq, Eq)]
  |                     -- in this derive macro expansion
...
4 |     Float(Option<f64>),
  |           ^^^^^^^^^^^ the trait `Eq` is not implemented for `f64`
  |
  = help: the following other types implement trait `Eq`:
            i128
            i16
            i32
            i64
            i8
            isize
            u128
            u16
          and 4 others
  = note: required because of the requirements on the impl of `Eq` for `Option<f64>`
note: required by a bound in `AssertParamIsEq`
  = note: this error originates in the derive macro `Eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
