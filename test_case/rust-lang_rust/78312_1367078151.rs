
error: lifetime may not live long enough
 --> src/main.rs:2:5
  |
1 | fn foo(mut first: &u8, second: &u8) {
  |                   -            - let's call the lifetime of this reference `'1`
  |                   |
  |                   let's call the lifetime of this reference `'2`
2 |     first = second;
  |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
  |
help: consider introducing a named lifetime parameter
  |
1 | fn foo<'a>(mut first: &'a u8, second: &'a u8) {
  |       ++++             ++              ++
