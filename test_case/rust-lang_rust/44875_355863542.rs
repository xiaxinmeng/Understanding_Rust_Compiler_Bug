
error[E0623]: lifetime mismatch
 --> src/main.rs:9:9
  |
4 | fn foo2<'a, 'b>(a: &'a Foo, x: &'b i32) -> &'a i32 {
  |                                -------     -------
  |                                |
  |                                this parameter and the return type are declared with different lifetimes...
...
9 |         &*x
  |         ^^^ ...but data from `x` is returned here
