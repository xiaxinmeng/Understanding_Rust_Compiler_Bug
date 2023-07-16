
$ RUSTFLAGS="-Z macro-backtrace" cargo check
    Checking bloop v0.1.0 (/home/alex/bloop)
error[E0658]: arbitrary expressions in key-value attributes are unstable
  --> src/main.rs:3:61
   |
1  | / macro_rules! define_two_property_arithmetic_struct {
2  | |     ($name:ident) => {
3  | |         define_two_property_arithmetic_struct!(@IMPL $name, stringify!($name));
   | |                                                             ^^^^^^^^^^^^^^^^^
4  | |     };
...  |
12 | |     };
13 | | }
   | |_- in this expansion of `define_two_property_arithmetic_struct!`
14 |
15 |   define_two_property_arithmetic_struct!(Position);
   |   ------------------------------------------------- in this macro invocation
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable
