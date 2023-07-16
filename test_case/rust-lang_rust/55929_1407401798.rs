
error[E0521]: borrowed data escapes outside of function
 --> src/lib.rs:4:5
  |
3 | pub fn func<'a>(source: &'a str) {
  |             --  ------ `source` is a reference that is only valid in the function body
  |             |
  |             lifetime `'a` defined here
4 |     take(1).or(string("a")).parse(source);   // error
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     `source` escapes the function body here
  |     argument requires that `'a` must outlive `'static`

For more information about this error, try `rustc --explain E0521`.
