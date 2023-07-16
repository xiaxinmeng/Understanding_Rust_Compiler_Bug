
error[E0623]: lifetime mismatch
 --> src/main.rs:2:9
  |
1 | fn test<'a, 'b>(mut a: &'a i32, b: &'b i32) {
  |                        -------     ------- these two types are declared with different lifetimes...
2 |     a = b;
  |         ^ ...but data from `b` flows into `a` here
