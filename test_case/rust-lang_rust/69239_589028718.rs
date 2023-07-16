
error[E0658]: const generics are unstable
 --> src/lib.rs:1:19
  |
1 | trait Trait<const S: &'static str> {}
  |                   ^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/44580
