
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
 --> src/main.rs:5:17
  |
4 |       fn foo(&mut self) {
  |              --------- this data with an anonymous lifetime `'_`...
5 |           let x = move || {
  |  _________________^
6 | |             drop(self);
7 | |         };
  | |_________^ ...is captured here...
8 |           foo(x);
  |           --- ...and is required to live as long as `'static` here
