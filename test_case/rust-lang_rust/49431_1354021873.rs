
error: lifetime may not live long enough
 --> src/main.rs:4:5
  |
3 | fn foobar<'a, 'b>(foo: &'a str, bar: &'b str) -> impl Debug + 'a + 'b {
  |           --  -- lifetime `'b` defined here
  |           |
  |           lifetime `'a` defined here
4 |     (foo, bar)
  |     ^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
  |
  = help: consider adding the following bound: `'a: 'b`

error: lifetime may not live long enough
 --> src/main.rs:4:5
  |
3 | fn foobar<'a, 'b>(foo: &'a str, bar: &'b str) -> impl Debug + 'a + 'b {
  |           --  -- lifetime `'b` defined here
  |           |
  |           lifetime `'a` defined here
4 |     (foo, bar)
  |     ^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
  |
  = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
