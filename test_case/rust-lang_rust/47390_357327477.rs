
warning: unused variables: `field` & `field2`
 --> src/main.rs:8:25
  |
8 |   if let Enum::Variant {field, field2, field3} = whatever {
  |                         ^^^^^  ^^^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = help: to avoid this warning, consider using `..` instead:
  |
8 |   if let Enum::Variant {field3, ..} = whatever {
  |                         ^^^^^^^^^^
