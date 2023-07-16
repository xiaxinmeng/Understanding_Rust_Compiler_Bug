rust
error: no method named `f` found for type `{integer}` in the current scope
 --> src/lib.rs:5:11
  |
5 |         3.f()
  |           ^
  |
  = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: candidate #1 is defined in the trait `b::Tr`
 --> b/src/lib.rs:2:5
  |
2 | 
  |  _____^ starting here...
3 | |
  | |__________________________^ ...ending here
  = help: to disambiguate the method call, write `b::Tr::f(3)` instead

error: aborting due to previous error

error: Could not compile `a`.

To learn more, run the command again with --verbose.
