
error: lifetime may not live long enough
 --> src/main.rs:2:5
  |
1 | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
  |          --  -- lifetime `'c` defined here
  |          |
  |          lifetime `'b` defined here
2 |     *z = *y;
  |     ^^^^^^^ assignment requires that `'b` must outlive `'c`
  |
  = help: consider adding the following bound: `'b: 'c`
