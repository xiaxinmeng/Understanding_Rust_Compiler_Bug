
error[E0518]: attribute should be applied to function
  --> examples/E0518.rs:14:1
   |
14 | #[inline(never)] //~ ERROR E0518
   | ^^^^^^^^^^^^^^^^ requires a function

error[E0517]: attribute should be applied to struct or union
  --> examples/E0517.rs:14:1
   |
14 | #[repr(packed)] //~ requires a struct
   | ^^^^^^^^^^^^^^^ requires a struct or union

error[E0084]: unsupported representation for zero-variant enum
  --> examples/E0084.rs:12:1
   |
12 | enum Foo {}  //~ ERROR E0084
   | ^^^^^^^^^^^ unsupported enum representation
