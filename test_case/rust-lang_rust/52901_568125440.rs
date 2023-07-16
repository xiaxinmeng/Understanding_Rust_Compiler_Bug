
error[E0049]: method `foo` has 1 type parameter but its trait declaration has 0 type parameters
  --> src/main.rs:12:25
   |
2  |     fn foo(&self, i32) {}
   |           - expected 0 type parameters
...
12 |     fn foo(&self, bar: &impl FooTrait) {}
   |                         ^^^^^^^^^^^^^
   |                         |
   |                         found 1 type parameter
   |                         `impl Trait` introduces an implicit type parameter

error[E0049]: method `bar` has 1 type parameter but its trait declaration has 0 type parameters
  --> src/main.rs:13:25
   |
3  |     fn bar(&self, i32) {}
   |           - expected 0 type parameters
...
13 |     fn bar(&self, bar: &impl FooTrait) {}
   |                         ^^^^^^^^^^^^^
   |                         |
   |                         found 1 type parameter
   |                         `impl Trait` introduces an implicit type parameter

error: aborting due to 2 previous errors
