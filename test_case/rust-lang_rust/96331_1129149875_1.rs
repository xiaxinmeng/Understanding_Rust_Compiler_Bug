
error[E0623]: lifetime mismatch
 --> src/main.rs:2:10
  |
1 | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
  |                          ---------          --------- these two types are declared with different lifetimes...
2 |     *x = *y;
  |          ^^ ...but data from `y` flows into `x` here

error[E0623]: lifetime mismatch
 --> src/main.rs:3:10
  |
1 | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
  |                                             ---------          ---------
  |                                             |
  |                                             these two types are declared with different lifetimes...
2 |     *x = *y;
3 |     *z = *y;
  |          ^^ ...but data from `y` flows into `z` here

error: aborting due to 2 previous errors
