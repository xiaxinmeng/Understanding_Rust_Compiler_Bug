
error[E0518]: attribute should be applied to function
  --> examples/E0518.rs:14:1
   |
14 |   #[inline(never)] //~ ERROR E0518
   |   ^^^^^^^^^^^^^^^^
15 | / impl Foo {       //~ not a function
16 | | }
   | |_- not a function

error[E0517]: attribute should be applied to struct or union
  --> examples/E0517.rs:14:1
   |
14 | #[repr(packed)] //~ requires a struct
   | ^^^^^^^^^^^^^^^
15 | enum Foo2 {Bar, Baz} //~ ERROR E0517
   | -------------------- not a struct or union

error[E0084]: unsupported representation for zero-variant enum
  --> examples/E0084.rs:11:1
   |
11 | #[repr(i32)] //~ unsupported enum representation
   | ^^^^^^^^^^^^
12 | enum Foo {}  //~ ERROR E0084
   | ----------- zero-variant enum
