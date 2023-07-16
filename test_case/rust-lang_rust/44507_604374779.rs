
error: lifetime may not live long enough
 --> src/main.rs:4:5
  |
3 | fn foo<'a, 'b>(mut x: Vec<&'a u32>, y: &'b u32)  {
  |        --  -- lifetime `'b` defined here
  |        |
  |        lifetime `'a` defined here
4 |     x.push(y);
  |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`
  |
  = help: consider adding the following bound: `'b: 'a`
