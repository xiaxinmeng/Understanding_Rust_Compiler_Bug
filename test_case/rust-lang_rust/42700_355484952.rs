
error[E0623]: lifetime mismatch
 --> src/main.rs:9:15
  |
7 | fn foo<'a>(&self, x: &'a Foo) -> &'a Foo {
  |            -----                 -------
  |            |
  |            this parameter and the return type are declared with different lifetimes...
8 | 
9 |     if true { self } else { x }
  |               ^^^^ ...but data from `self` is returned here
