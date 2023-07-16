
error: expected identifier
  --> src/lib.rs:15:17
   |
15 |                   $field_vis $field: $type,
   |                   ^^^^^^^^^^
...
21 | / regurgitate_struct_def! {
22 | |     #[derive(serde_derive::Deserialize)]
23 | |     pub struct Foo {
24 | |         _phantom: (),
25 | |     }
26 | | }
   | |_- in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

error: could not compile `nightlytest`.
