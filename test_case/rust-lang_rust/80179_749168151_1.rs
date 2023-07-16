rust
error[E0308]: mismatched types
 --> <anon>:2:16
  |
2 | fn g() -> () { f }
  |           --   ^ expected `()`, found fn item
  |           |
  |           expected `()` because of return type
  |
  = note: expected unit type `()`
               found fn item `fn() -> i32 {f}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
 