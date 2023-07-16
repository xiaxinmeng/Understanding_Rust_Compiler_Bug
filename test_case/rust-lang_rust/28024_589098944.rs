
error: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions.
 --> src/lib.rs:5:9
  |
5 | fn show<T: Debug = Foo>(t: T) {
  |         ^
  |
  = note: `#[deny(invalid_type_param_default)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #36887 <https://github.com/rust-lang/rust/issues/36887>
