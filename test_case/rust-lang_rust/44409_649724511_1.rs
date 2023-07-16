

error[E0623]: lifetime mismatch
 --> src/main.rs:2:5
  |
1 | fn test<'a, 'b: 'a>(x: &'a mut &'b mut i32) -> &'b mut i32 {
  |                        -------------------
  |                        |
  |                        these two types are declared with different lifetimes...
2 |     &mut **x
  |     ^^^^^^^^ ...but data from `x` flows into `x` here

