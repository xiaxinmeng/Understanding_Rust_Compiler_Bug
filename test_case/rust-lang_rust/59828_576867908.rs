
error: comparison operators cannot be chained
 --> src/lib.rs:2:7
  |
2 |     x < y ^ y < z // error: chained comparison operators require parentheses
  |       ^^^^^^^^^
  |
  = help: use `::<...>` instead of `<...>` to specify type arguments
  = help: or use `(...)` if you meant to specify fn arguments
help: split the comparison into two...
  |
2 |     x < y ^ y && y ^ y < z // error: chained comparison operators require parentheses
  |     ^^^^^^^^^^^^^^^^^^^^
help: ...or parenthesize one of the comparisons
  |
2 |     (x < y ^ y) < z // error: chained comparison operators require parentheses
  |     ^^^^^^^^^^^^^
