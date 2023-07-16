
error[E0038]: the trait `Foo` cannot be made into an object
  --> file.rs:11:16
   |
1  | trait Foo {
   |       --- this trait cannot be made into an object...
2  |   fn apply() -> i64;
   |      ----- ...because associated function `apply` has no `self` parameter
...
11 | fn lookup() -> Box<&'static Foo> {
   |                ^^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
   |
help: consider turning `apply` into a method by giving it a `&self` argument or constraining it so it does not apply to trait objects
   |
2  |   fn apply() -> i64 where Self: Sized;
   |                     ^^^^^^^^^^^^^^^^^
