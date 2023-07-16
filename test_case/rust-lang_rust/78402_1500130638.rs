
error[[E0521]](https://doc.rust-lang.org/stable/error_codes/E0521.html): borrowed data escapes outside of function
 --> src/lib.rs:7:5
  |
6 | async fn foo(a: &str) {
  |              -  - let's call the lifetime of this reference `'1`
  |              |
  |              `a` is a reference that is only valid in the function body
7 |     assert_static(bar(a.split(',')));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     `a` escapes the function body here
  |     argument requires that `'1` must outlive `'static`

For more information about this error, try `rustc --explain E0521`.
error: could not compile `playground` due to previous error
