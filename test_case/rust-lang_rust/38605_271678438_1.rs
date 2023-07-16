
error[E0308]: mismatched types
 --> /home/nmatsakis/tmp/foo.rs:1:8
  |
1 | fn agh(&&bar: u32) {
  |        ^^^^^ expected u32, found reference
  |
  = note: expected type `u32`
  = note:    found type `&_`
  = help: did you mean `&bar: &u32`?

error: aborting due to previous error
