
[00:35:42] error[E0308]: mismatched types
[00:35:42]    --> /checkout/src/librustdoc/test.rs:260:31
[00:35:42]     |
[00:35:42] 260 |         driver::compile_input(&sess, &cstore, &input, &out, &None, None, &control)
[00:35:42]     |                               ^^^^^ types differ in mutability
[00:35:42]     |
[00:35:42]     = note: expected type `&mut rustc::session::Session`
[00:35:42]                found type `&rustc::session::Session`
