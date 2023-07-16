
Compiling playground v0.0.1 (/playground)
error[[E0521]](https://doc.rust-lang.org/stable/error_codes/E0521.html): borrowed data escapes outside of associated function
  --> src/main.rs:12:13
   |
10 | impl <'s> S<'s> {
   |       -- lifetime `'s` defined here
11 |     fn f(self) {
   |          ---- `self` is a reference that is only valid in the associated function body
12 |         foo(self.x.unwrap_or_else(bar)) // broken
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |             |
   |             `self` escapes the associated function body here
   |             argument requires that `'s` must outlive `'static`

For more information about this error, try `rustc --explain E0521`.
error: could not compile `playground` due to previous error
