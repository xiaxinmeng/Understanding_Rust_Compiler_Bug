
error: recursion limit reached while expanding the macro `a`
 --> test.rs:2:13
  |
2 |     () => { a!() }
  |             ^^^^
...
6 |     a!()
  |     ---- in this macro invocation
  |
  = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
