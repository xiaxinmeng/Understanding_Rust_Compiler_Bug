plain
ii.Fi..ii......i......i.ii..i........i.....iii..i...................i........i..i....... 88/111
.......i...............
failures:

---- src/builtin.rs - builtin::DEPRECATED_CFG_ATTR_CRATE_TYPE_NAME (line 3070) stdout ----
error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
  |
2 | #![cfg_attr(debug_assertions, crate_type = "lib")]
  |                               ^^^^^^^^^^^^^^^^^^
  |
  |
  = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` on by default
  = note: for more information, see issue #91632 <https://github.com/rust-lang/rust/issues/91632>

error: aborting due to previous error

