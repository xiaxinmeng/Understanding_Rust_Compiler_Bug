
error[E0322]: explicit impls for the `Sized` trait are not permitted
 --> src/main.rs:7:1
  |
7 | / impl Sized for u32 {
8 | | }
  | |_^ impl of 'Sized' not allowed

error[E0322]: explicit impls for the `Sized` trait are not permitted
  --> src/main.rs:9:1
   |
9  | / impl Sized for u64 {
10 | | }
   | |_^ impl of 'Sized' not allowed
