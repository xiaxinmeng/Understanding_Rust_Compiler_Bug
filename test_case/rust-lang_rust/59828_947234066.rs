
   Compiling playground v0.0.1 (/playground)
error: comparison operators cannot be chained
 --> src/lib.rs:2:7
  |
2 |     x < y ^ y < z
  |       ^       ^
  |
  = help: use `::<...>` instead of `<...>` to specify type or const arguments
  = help: or use `(...)` if you meant to specify fn arguments
help: split the comparison into two
  |
2 |     x < y ^ y && y ^ y < z
  |               ++++++++

error: could not compile `playground` due to previous error
