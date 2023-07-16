
$ cargo build
   Compiling doryen-extra v0.1.0 (/home/alex/doryen-extra)
error[E0658]: arbitrary expressions in key-value attributes are unstable
  --> src/base/def_macro.rs:35:77
   |
35 | ...   define_two_property_arithmetic_struct!(@IMPL $name, $uname, $fname, stringify!($name), $field1, $field2, stringify!($field1), strin...
   |                                                                           ^^^^^^^^^^^^^^^^^
   |
  ::: src/base.rs:39:1
   |
39 | define_two_property_arithmetic_struct!(Position, UPosition, FPosition, x, y, ORIGIN, "({}, {})");
   | ------------------------------------------------------------------------------------------------- in this macro invocation
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: arbitrary expressions in key-value attributes are unstable
  --> src/base/def_macro.rs:35:77
   |
35 | ...   define_two_property_arithmetic_struct!(@IMPL $name, $uname, $fname, stringify!($name), $field1, $field2, stringify!($field1), strin...
   |                                                                           ^^^^^^^^^^^^^^^^^
   |
  ::: src/base.rs:40:1
   |
40 | define_two_property_arithmetic_struct!(Size, USize, FSize, width, height, ZERO, "{}x{}");
   | ----------------------------------------------------------------------------------------- in this macro invocation
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
