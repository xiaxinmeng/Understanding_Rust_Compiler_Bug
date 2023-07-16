
warning: unused variable: `field`
 --> src/main.rs:8:25
  |
8 |   if let Enum::Variant {field, field2, field3} = whatever {
  |                         ^^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_field` instead

warning: unused variable: `field2`
 --> src/main.rs:8:32
  |
8 |   if let Enum::Variant {field, field2, field3} = whatever {
  |                                ^^^^^^
  |
  = note: to avoid this warning, consider using `_field2` instead
