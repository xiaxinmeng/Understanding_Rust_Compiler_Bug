
error[E0597]: lifetime error
  --> src/main.rs:9:14
   |
8 | fn foo(x: &u32) -> &u32 {
  |            -
  |            | let's call this `'1`
9 |     &y
   |          ^ `y` would have to be valid for `'1`
10| }
  | - but `y` goes out of scope here
