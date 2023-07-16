
error[E0616]: field `len` of struct `foo::Foo` is private
 --> src/main.rs:3:8
  |
3 |     if x.len {
  |        ^^^^^ help: a method `len` also exists, perhaps you wish to call it: `x.len()`

error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if x.len {
  |        ^^^^^ expected bool, found `usize`
  |
  = note: expected type `bool`
             found type `usize`
  = note: found `usize` assuming that `x.len` has been rewritten to `x.len()`
