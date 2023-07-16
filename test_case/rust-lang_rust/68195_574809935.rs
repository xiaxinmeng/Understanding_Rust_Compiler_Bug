
error[E0038]: the trait `NotObjectSafe` cannot be made into an object
  --> src/test/ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait.rs:21:1
   |
3  |     fn foo() -> Self;
   |        --- associated function `foo` has no `self` parameter
...
21 | fn car() -> dyn NotObjectSafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object

error[E0038]: the trait `NotObjectSafe` cannot be made into an object
  --> src/test/ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait.rs:28:1
   |
3  |     fn foo() -> Self;
   |        --- associated function `foo` has no `self` parameter
...
28 | fn cat() -> Box<dyn NotObjectSafe> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
