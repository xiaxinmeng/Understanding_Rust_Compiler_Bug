
error[E0038]: the trait `Foo` cannot be made into an object
  --> src/lib.rs:11:1
   |
11 | fn lookup() -> Box<&'static Foo> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
   |
   = note: method `apply` has no receiver
