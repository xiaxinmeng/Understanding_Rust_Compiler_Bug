
error[E0038]: the trait `Foo` cannot be made into an object
  --> src/lib.rs:11:1
   |
2  |   fn apply() -> i64;
   |      ----- associated function `apply` has no `self` parameter
...
11 | fn lookup() -> Box<&'static Foo> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
