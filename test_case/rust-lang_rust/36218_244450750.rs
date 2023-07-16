
error[E0106]: missing lifetime specifier
  --> src/main.rs:11:1
   |
11 | struct A {
   | ^ expected lifetime parameter

error[E0038]: the trait `T` cannot be made into an object
  --> src/main.rs:16:1
   |
16 | struct B<'a> {
   | ^ the trait `T` cannot be made into an object
