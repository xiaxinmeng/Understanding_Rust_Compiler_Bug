
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the trait bound `Compact<T>: Decode` is not satisfied
  --> src/main.rs:35:22
   |
35 | fn bar<T>() { _foo::<T>() }
   |                      ^ the trait `Decode` is not implemented for `Compact<T>`
   |
   = help: the trait `Decode` is implemented for `Compact<T>`
note: required for `T` to implement `HasCompact`
  --> src/main.rs:22:9
   |
22 | impl<T> HasCompact for T
   |         ^^^^^^^^^^     ^
23 | where
24 |     Compact<T>: Decode,
   |                 ------ unsatisfied trait bound introduced here
note: required by a bound in `_foo`
  --> src/main.rs:33:12
   |
33 | fn _foo<T: HasCompact>() {}
   |            ^^^^^^^^^^ required by this bound in `_foo`
