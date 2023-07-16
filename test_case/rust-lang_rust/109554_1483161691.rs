
error: range endpoint is out of range for `u8`
  --> $DIR/deny-overflowing-literals.rs:5:14
   |
LL |     for _ in 0..256u8 {}
   |              ^^^^^^^^
   |
help: use an inclusive range instead
   |
LL |     for _ in 0..=255u8 {}
   |                 ~~~~~
