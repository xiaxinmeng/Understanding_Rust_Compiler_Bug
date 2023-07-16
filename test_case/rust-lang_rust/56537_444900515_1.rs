
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
 --> free-willy.rs:2:41
  |
2 |     let free_dumb = |x: &str| -> &str { p };
  |                                         ^
  |
note: ...the reference is valid for the anonymous lifetime #1 defined on the body at 2:21...
 --> free-willy.rs:2:21
  |
2 |     let free_dumb = |x: &str| -> &str { p };
  |                     ^^^^^^^^^^^^^^^^^^^^^^^
note: ...but the borrowed content is only valid for the lifetime 'w as defined on the function body at 1:10
 --> free-willy.rs:1:10
  |
1 | fn willy<'w>(p: &'w str) -> &'w str {
  |          ^^
