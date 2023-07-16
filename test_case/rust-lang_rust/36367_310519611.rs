
error: unexpected token: `::`
 --> test.rs:3:38
  |
3 |     if i as usize < std::mem::size_of::<i64>() {
  |                                      ^^
  |
  = help: use `<...>` instead of `::<...>` if you meant to specify type arguments

error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
 --> test.rs:3:45
  |
3 |     if i as usize < std::mem::size_of::<i64>() {
  |                   -                         ^ interpreted as generic argument
  |                   |
  |                   not interpreted as comparison
  |
help: if you want to compare the casted value then write:
  |     if (i as usize) < std::mem::size_of::<i64>() {

error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
 --> test.rs:6:21
  |
6 |     if i as usize < 5 {
  |                   - ^ interpreted as generic argument
  |                   |
  |                   not interpreted as comparison
  |
help: if you want to compare the casted value then write:
  |     if (i as usize) < 5 {

error: aborting due to previous error(s)
