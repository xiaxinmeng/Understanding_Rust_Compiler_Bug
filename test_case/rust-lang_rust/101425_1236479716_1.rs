
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): `()` doesn't implement `std::fmt::Display`
 --> src/main.rs:4:13
  |
4 |     let x = foo::<()>;
  |             ^^^^^^^^^ `()` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `foo`
 --> src/main.rs:1:11
  |
1 | fn foo<T: std::fmt::Display>() {}
  |           ^^^^^^^^^^^^^^^^^ required by this bound in `foo`
