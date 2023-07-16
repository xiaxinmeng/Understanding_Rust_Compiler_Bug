
error[E0573]: expected type, found variant `Some`
 --> src/main.rs:5:13
  |
5 | fn foo() -> Some(usize) {
  |             ^^^^^^^^^^^ not a type
help: try using the variant's enum
  |
5 | fn foo() -> core::prelude::v1::Option {
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^
5 | fn foo() -> core::prelude::v1 {
  |             ^^^^^^^^^^^^^^^^^
5 | fn foo() -> lzw::Bits {
  |             ^^^^^^^^^
5 | fn foo() -> serde::export::Option {
  |             ^^^^^^^^^^^^^^^^^^^^^
and 4 other candidates
