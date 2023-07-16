
error: lifetime may not live long enough
 --> src/main.rs:2:5
  |
1 | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
  |      --  -- lifetime `'b` defined here
  |      |
  |      lifetime `'a` defined here
2 |     *x = *y;
  |     ^^^^^^^ assignment requires that `'b` must outlive `'a`
  |
  = help: consider adding the following bound: `'b: 'a`

error: aborting due to previous error
