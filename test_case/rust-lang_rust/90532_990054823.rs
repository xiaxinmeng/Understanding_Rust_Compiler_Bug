rust
error[E0015]: cannot call non-const fn `<[closure@$DIR/issue-28113.rs:4:5: 4:19] as Fn<()>>::call` in constants
  --> $DIR/issue-28113.rs:4:5
   |
LL |     || -> u8 { 5 }()
   |     ^^^^^^^^^^^^^^^^
   |
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
