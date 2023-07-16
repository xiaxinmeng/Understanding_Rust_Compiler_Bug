rust
error: Rust has no postfix increment operator
 --> ./p/inc.rs:8:9
  |
8 |     if i++ == 1 {}
  |         ^^ not a valid postfix operator
  |
help: use `+= 1` instead
  |
8 |     if { let tmp = i; i += 1; tmp } == 1 {}
  |        +++++++++++  ~~~~~~~~~~~~~~~
8 |     if i += 1 == 1 {}
  |          ~~~~
