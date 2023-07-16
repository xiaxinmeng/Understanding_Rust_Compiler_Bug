
error[E0623]: lifetime mismatch
  --> src/lib.rs:12:36
   |
6  | fn foo<'a>(&self, x: &'a i32) -> &i32 {
   |                      -------     ----
   |                      |
   |                      this parameter and the return type are declared with different lifetimes...
...
12 |     if true { &self.field } else { x }
   |                                    ^ ...but data from `x` is returned here

error[E0623]: lifetime mismatch
  --> src/lib.rs:19:5
   |
15 |   fn bar<'a>(&self, x: &'a i32) -> &i32 {
   |                        -------     ----
   |                        |
   |                        this parameter and the return type are declared with different lifetimes...
...
19 |     x
   |     ^ ...but data from `x` is returned here
