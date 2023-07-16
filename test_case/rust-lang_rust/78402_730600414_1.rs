
error[E0621]: explicit lifetime required in the type of `a`
 --> src/lib.rs:7:5
  |
6 | fn foo(a: &str) {
  |           ---- help: add explicit lifetime `'static` to the type of `a`: `&'static str`
7 |     assert_static(bar(a.split(',')));
  |     ^^^^^^^^^^^^^ lifetime `'static` required
