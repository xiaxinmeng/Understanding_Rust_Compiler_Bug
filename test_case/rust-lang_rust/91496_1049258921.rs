
error[E0015]: cannot perform deref coercion on `SmartPointer` in constant functions
  --> src/lib.rs:21:5
   |
21 |     SmartPointer(Foo).foo(); // fails since we can't constly deref
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: attempting to deref into `Foo`
note: deref defined here
  --> src/lib.rs:12:5
   |
12 |     type Target = Foo;
   |     ^^^^^^^^^^^^^^^^^^
note: impl defined here, but it is not `const`
  --> src/lib.rs:11:1
   |
11 | impl Deref for SmartPointer {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

