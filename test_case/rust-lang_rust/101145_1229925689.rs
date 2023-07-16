
error[E0594]: cannot assign to `self.parse_header`, which is behind a `&` reference
 --> src/lib.rs:6:9
  |
5 |     fn set_include_mode(&self, parse_header: bool) {
  |                         ----- help: consider changing this to be a mutable reference: `&mut self`
6 |         self.parse_header = parse_header;
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be written
