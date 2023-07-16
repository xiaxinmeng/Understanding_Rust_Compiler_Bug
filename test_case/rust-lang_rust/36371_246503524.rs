
error[E0186]: method `foo` has a `&self` declaration in the trait, but not in the impl
  --> src/test/compile-fail/E0186.rs:18:5
   |
12 |     fn foo(&self);
   |     -------------- `&self` used in trait
...
18 |     fn foo() {}
   |     ^^^^^^^^^^^ expected `&self` in impl
